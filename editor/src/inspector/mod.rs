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
use std::collections::{HashMap, HashSet};
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

    pub entities: Vec<Entity>,
    pub components: HashMap<Entity, Vec<ComponentId>>,

    pub tab_info: TabInfo,
    pub z: ggez::graphics::ZIndex,

    /// (`ID`, `Name`)
    ///
    /// `String` = scene name
    pub focused_entity: Option<(Entity, String)>,
    pub focused_component: Option<ComponentId>,
    pub component_modules: HashMap<String, Vec<((String, String), bevy_reflect::TypeRegistration)>>,

    pub debug_mode: bool,
}

impl EditorGUI {
    pub(crate) fn new(ctx: &mut ggez::Context, world: &mut World) -> Self {
        let mut entities = Vec::new();
        let mut components = HashMap::new();

        for (entity, scene_data) in world
            .query::<(Entity, &crate::scene::SceneData)>()
            .iter(&world)
        {
            entities.push(entity);
        }

        for (entity, dyn_components) in world
            .query::<(Entity, &dyn crate::scene::TestSuperTrait)>()
            .iter(&world)
        {
            components.insert(
                entity,
                dyn_components
                    .iter()
                    .map(|component| {
                        world
                            .components()
                            .get_id(component.as_reflect().type_id())
                            .unwrap()
                    })
                    .collect::<Vec<ComponentId>>(),
            );
        }

        let types = world
            .resource::<SceneManager>()
            .type_registry
            .iter()
            .filter(|i| i.data::<crate::scene::ReflectTestSuperTrait>().is_some());

        let mut modules: HashMap<String, Vec<((String, String), bevy_reflect::TypeRegistration)>> =
            HashMap::new();

        for type_ in types {
            let full_path = type_.type_info().type_path().to_string();

            if let Some(index) = full_path.find("::") {
                let split = (
                    full_path.split_at(index).0.to_owned(), // the module name
                    full_path
                        .split_at(index)
                        .1
                        .strip_prefix("::")
                        .unwrap()
                        .to_owned(), // the other part of the component path, including the name
                );

                if (&modules.get_mut(&split.0)).is_some() {
                    modules
                        .get_mut(&split.0)
                        .unwrap()
                        .push(((full_path, split.1), type_.clone()));
                } else {
                    modules.insert(split.0, vec![((full_path, split.1), type_.clone())]);
                };
            }
        }

        Self {
            entities,
            components,

            focused_entity: None,
            focused_component: None,
            component_modules: modules,

            debug_mode: false,
            tab_info: TabInfo::default(),
            z: 1000,
            gui: ggegui::Gui::new(ctx),
            dock_state: DockState::new(vec![]),
        }

        // Self {

        //     entities: Vec::new(),
        //     components: HashMap::new(),

        //     tab_info: TabInfo::default(),

        //     z: 0,
        //     focused_entity: None,
        //     focused_component: None,
        //     component_modules: HashMap::new(),
        //     debug_mode: false,
        // }
    }

    fn switch_to_tab(&mut self, tab_id: egui::Id) {
        let mut tab_index = None;

        for (_, tab) in self.dock_state.iter_all_tabs() {
            if tab.id == tab_id {
                tab_index = self.dock_state.find_tab(tab);
            }
        }

        if let None = tab_index {
            log::error!("Tab {:?} could not be switched to", tab_id);
            return;
        };

        self.dock_state.set_active_tab(tab_index.unwrap());
    }
}

impl egui_dock::TabViewer for EditorGUI {
    type Tab = EditorTabState;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        todo!()
    }

    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        // if let Some(focused_entity) = self.focused_entity.clone() {
        //     let mut v: Vec<std::any::TypeId> = Vec::new();
        //     for (entity, read) in self
        //         .world_mut()
        //         .query::<(Entity, &dyn crate::scene::TestSuperTrait)>()
        //         .iter(self.world_mut())
        //     {
        //         if !(entity == focused_entity.0) {
        //             continue;
        //         }

        //         v = read
        //             .iter()
        //             .map(|component| component.as_reflect().type_id())
        //             .collect();
        //     }

        //     let v = v
        //         .iter()
        //         .map(|component| self.world_mut().components().get_id(*component).unwrap())
        //         .collect();

        //     self.components.insert(focused_entity.0, v);
        // }

        if let Some(response) = tab.state.ui(self, ui) {
            match response {
                TabResponse::SwitchToTab(tab_id) => self.switch_to_tab(tab_id),
                TabResponse::RemoveComponent(_, _) => todo!(),
            }
        }
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

                ui.add(egui::DragValue::new(&mut editor_ui.window.z));
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
        let tab = T::create_tab();

        let tab_id = tab.id.clone();

        editor_resource.dock_state.add_window(vec![tab]);

        editor_resource
            .window
            .tab_info
            .tab_focused(T::name().to_owned(), tab_id);

        label_ui.close_menu();
    }
}

pub fn draw_editor_gui(editor: Res<EditorGUI>, mut engine: ResMut<GgezInterface>) {
    engine.get_canvas_mut().unwrap().draw(
        &editor.gui,
        DrawParam::default().dest([0.0, 0.0]).z(editor.window.z),
    );
}
