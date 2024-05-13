use self::entity_view::EntityHeirarchyTab;
use self::field_view::FieldInspectTab;
use self::inspector_view::InspectorTab;
use bevy_ecs::component::ComponentId;
use bevy_ecs::prelude::*;
use bevy_reflect::{reflect_trait, Reflect, TypeRegistry};
use egui::{Pos2, Ui};
use egui_dock::{DockArea, DockState, Style, SurfaceIndex};
use engine::editor::*;
use engine::editor::{EditorTab, InspectableAsField};
use engine::scene::{ReflectTestSuperTrait, SceneData, SceneManager, TestSuperTrait};
use engine::{GgezInterface, Input};
use ggez::graphics::{Canvas, DrawParam};
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::DerefMut;

pub mod entity_view;
pub mod field_view;
pub mod game_view;
pub mod inspector_view;
pub mod scene_window;

/// Container for all of the state relating to the [`egui`] GUI.
#[derive(Resource)]
pub struct EditorGUI
where
    Self: 'static,
    Self: Send,
    Self: Sync,
{
    gui: ggegui::Gui,
    dock_state: egui_dock::DockState<EditorTabState>,
    current_response: Option<TabResponse>,
    window: WindowState,
    /// These aren't displayed or updated, but will be used to clone and instantiate new windows
    registered_window_types: Vec<Box<dyn EditorTab>>,
}

impl EditorGUI {
    pub(crate) fn new(ctx: &mut ggez::Context, world: &mut World) -> Self {
        Self {
            gui: ggegui::Gui::new(ctx),
            dock_state: DockState::new(vec![EntityHeirarchyTab::create_tab()]),
            current_response: None,
            window: WindowState::new(world),
            registered_window_types: Vec::new(),
        }
    }

    pub fn register_window<T: EditorTab>(&mut self) {
        self.registered_window_types.push(T::default_boxed());
    }

    fn inspector_dock_ui<'a>(&mut self, ui: &mut Ui, world: &'a mut World) {
        DockArea::new(&mut self.dock_state)
            .style(Style::from_egui(ui.style().as_ref()))
            .show_inside(ui, &mut self.window);

        if let Some(response) = &self.current_response {
            match response {
                TabResponse::SwitchToTab(new_tab) => {
                    todo!()
                    // let node = self
                    //     .dock_state
                    //     .get_surface(SurfaceIndex::main())
                    //     .unwrap()
                    //     .node_tree()
                    //     .unwrap()
                    //     .find_tab(&new_tab)
                    //     .unwrap();

                    // self.dock_state
                    //     .set_active_tab((SurfaceIndex::main(), node.0, node.1));
                }
                // TabResponse::RemoveComponent(entity, component_id) => {
                //     world.
                // }
                #[allow(unreachable_patterns)]
                // so that if there is a new tab response, I don't have to deal with this right away. Let it fail I say.
                // TODO: Remove if this project gains significance lol
                _ => unimplemented!(),
            }
        }
        self.current_response = self.window.current_response.take();
    }
}

pub fn update_windows<'a>(world: &mut World) {
    world.resource_scope(|world_scoped: &mut World, mut editor_ui: Mut<EditorGUI>| {
        unsafe { engine::editor::set_world_raw_pointer(Some(world_scoped)) };

        for key in world_scoped
            .resource_mut::<Input>()
            .iter_editor_keys_from_events()
        {
            match key {
                engine::input::key::keycode_converter::ButtonEvent::Scroll { x, y } => {
                    // TODO: Get system scroll amount and set it here
                    editor_ui.gui.input.mouse_wheel_event(x * 10.0, y * 10.0) // for some reason, ggez or ggegui is kinda funky with it's scroll events. So this just makes it easier to use.
                                                                              // TODO: Figure out that bug and alter this so this buggy workaround can be removed.
                }
                engine::input::key::keycode_converter::ButtonEvent::Text(ch) => {
                    editor_ui.gui.input.text_input_event(ch)
                }
            }
        }

        let ctx = editor_ui.gui.ctx();

        egui::TopBottomPanel::top("Menu").show(&ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.label("Menu");

                egui::menu::menu_button(ui, "File", |ui| file_managment_ui(ui));
                egui::menu::menu_button(ui, "Window", |ui| window_managment_ui(ui, &mut editor_ui));

                ui.checkbox(&mut editor_ui.window.debug_mode, "Debug mode");
            });
        });

        // egui::panel::SidePanel::left("LeftPanel").show(&ctx, |ui| {});
        // egui::panel::SidePanel::right("RightPanel").show(&ctx, |ui| {});

        let deref_mut = editor_ui.deref_mut(); // change detection tripped here regardless of actual changes since splitting borrows doesn't work with `Mut<EditorGUI>`

        DockArea::new(&mut deref_mut.dock_state)
            .window_bounds(ctx.screen_rect())
            .show(&ctx, &mut deref_mut.window);

        ggegui::Gui::update(
            &mut editor_ui.gui,
            world_scoped
                .resource_mut::<GgezInterface>()
                .get_context_mut(),
        );

        unsafe { engine::editor::set_world_raw_pointer(None) }
    });
}

fn file_managment_ui(ui: &mut Ui) {
    ui.selectable_label(false, "Save scene");
}

fn window_managment_ui(ui: &mut Ui, editor_resource: &mut Mut<EditorGUI>) {
    add_tab_label_ui::<inspector_view::InspectorTab>(ui, editor_resource);
    add_tab_label_ui::<entity_view::EntityHeirarchyTab>(ui, editor_resource);
    add_tab_label_ui::<field_view::FieldInspectTab>(ui, editor_resource);
    // add_tab_label_ui::<game_view::GameView>(ui, editor_resource); // on hold until I can fix it later
    add_tab_label_ui::<scene_window::SceneEditorTab>(ui, editor_resource);
    add_tab_label_ui::<components::editor_windows::MeshEditorTab>(ui, editor_resource);
    // ui.menu_button("Components", |ui| {
    // });
}

/// The UI for showing a selectable label that adds a tab when pressed
fn add_tab_label_ui<T: EditorTab>(label_ui: &mut Ui, editor_resource: &mut Mut<EditorGUI>) {
    let selectable_label = label_ui.button(T::name());

    if selectable_label.clicked() {
        editor_resource.dock_state.add_window(vec![T::create_tab()]);

        label_ui.close_menu();
    }
}

pub fn draw_editor_gui(editor: Res<EditorGUI>, mut engine: ResMut<GgezInterface>) {
    engine
        .get_canvas_mut()
        .unwrap()
        .draw(&editor.gui, DrawParam::default().dest([0.0, 0.0]).z(1000));
}
