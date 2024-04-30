use self::entity_view::EntityViewState;
use self::field_view::{FieldViewState, InspectorComponentField};
use self::inspector_view::InspectorViewState;
use engine::editor::InspectableAsField;
// use self::modname::InspectorData;
use bevy_ecs::component::ComponentId;
use bevy_ecs::prelude::*;
use bevy_reflect::{reflect_trait, Reflect, TypeRegistry};
use egui::{Pos2, Ui};
use egui_dock::{DockArea, DockState, Style, SurfaceIndex};
use engine::scene::{ReflectTestSuperTrait, SceneData, SceneManager, TestSuperTrait};
use engine::{GgezInterface, Input};
use ggez::graphics::DrawParam;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt::Debug;

pub mod entity_view;
pub mod field_view;
pub mod inspector_view;
mod scene_window;

#[derive(Debug)]
enum TabResponse {
    SwitchToTab(String),
    RemoveComponent(Entity, ComponentId),
}

///
#[derive(Resource)]
pub struct EditorInterface
where
    Self: 'static,
    Self: Send,
    Self: Sync,
{
    gui: ggegui::Gui,
    dock_state: egui_dock::DockState<String>,
    /// (`ID`, `Name`)
    ///
    /// `String` = entity name
    focused_entity: Option<(Entity, String)>,
    current_response: Option<TabResponse>,
    focused_component: Option<ComponentId>,
    window: InspectorWindow,
}

impl EditorInterface {
    pub(crate) fn new(ctx: &mut ggez::Context, world: &mut World) -> Self {
        let tabs = vec!["Entities", "Inspector", "Field", "Scene"]
            .drain(..)
            .map(str::to_string)
            .collect();

        let dock_state = DockState::new(tabs);

        Self {
            gui: ggegui::Gui::new(ctx),
            dock_state,
            focused_entity: None,
            focused_component: None,
            current_response: None,
            window: InspectorWindow::new(world),
        }
    }

    fn inspector_dock_ui<'a>(&mut self, ui: &mut Ui, world: &'a mut World) {
        DockArea::new(&mut self.dock_state)
            .style(Style::from_egui(ui.style().as_ref()))
            .show_inside(ui, &mut self.window);
        self.focused_entity = self.window.focused_entity.clone();
        self.focused_component = self.window.focused_component.clone();

        if let Some(response) = &self.current_response {
            match response {
                TabResponse::SwitchToTab(new_tab) => {
                    let node = self
                        .dock_state
                        .get_surface(SurfaceIndex::main())
                        .unwrap()
                        .node_tree()
                        .unwrap()
                        .find_tab(&new_tab)
                        .unwrap();

                    self.dock_state
                        .set_active_tab((SurfaceIndex::main(), node.0, node.1));
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

struct InspectorWindow
where
    Self: 'static,
    Self: Sync,
    Self: Send,
{
    pub entity_state: EntityViewState,
    pub inspector: InspectorViewState,
    pub field_state: FieldViewState,

    // "global" state (available in all inspector views)
    entities: Vec<Entity>,
    components: HashMap<Entity, Vec<ComponentId>>,
    current_response: Option<TabResponse>,

    /// (`ID`, `Name`)
    ///
    /// `String` = scene name
    pub focused_entity: Option<(Entity, String)>,
    pub focused_component: Option<ComponentId>,
    component_modules: HashMap<String, Vec<((String, String), bevy_reflect::TypeRegistration)>>,
    debug_mode: bool,
}

impl InspectorWindow {
    /// Requires access to InspectorWindow to get world access.
    ///
    /// Only works under specific circumstances.
    // TODO: Describe those circumstances here. Essentially, don't call outside of `draw_inspector()`
    pub(crate) fn world_ref(&self) -> &World {
        unsafe { &*(WORLD_REF.unwrap()) } // You called `world` when the reference wasn't available
    }

    /// Requires access to InspectorWindow to get world access.
    ///
    /// Only works under specific circumstances.
    // TODO: Describe those circumstances here. Essentially, don't call outside of `draw_inspector()`
    pub(crate) fn world_mut(&mut self) -> &mut World {
        unsafe { &mut *(WORLD_REF.unwrap()) } // You called `world` when the reference wasn't available
    }

    pub fn new(world: &mut World) -> Self {
        let mut entities = Vec::new();
        let mut components = HashMap::new();

        for (entity, scene_data) in world.query::<(Entity, &SceneData)>().iter(&world) {
            // if scene_data.hide_in_inspector {
            //     continue;
            // }

            entities.push(entity);
        }

        for (entity, dyn_components) in world.query::<(Entity, &dyn TestSuperTrait)>().iter(&world)
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

        // for entity in entities.values() {
        //     let bundle = InspectorData::new(world, *entity);
        //     world.entity_mut(*entity).insert(bundle);
        // }

        let types = world
            .resource::<SceneManager>()
            .type_registry
            .iter()
            .filter(|i| i.data::<ReflectTestSuperTrait>().is_some());

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
            inspector: InspectorViewState::default(),
            entity_state: EntityViewState::default(),
            field_state: FieldViewState::default(),

            entities,
            components,
            current_response: None,

            focused_entity: None,
            focused_component: None,
            component_modules: modules,

            debug_mode: false,
        }
    }
}

impl egui_dock::TabViewer for InspectorWindow {
    type Tab = String;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.to_string().into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        if let Some(focused_entity) = self.focused_entity.clone() {
            let mut v: Vec<TypeId> = Vec::new();
            for (entity, read) in self
                .world_mut()
                .query::<(Entity, &dyn TestSuperTrait)>()
                .iter(self.world_mut())
            {
                if !(entity == focused_entity.0) {
                    continue;
                }

                // TODO: Continue writing component update functionality, refactor component type if you must
                v = read
                    .iter()
                    .map(|component| component.as_reflect().type_id())
                    .collect();
            }

            let v = v
                .iter()
                .map(|component| self.world_mut().components().get_id(*component).unwrap())
                .collect();

            self.components.insert(focused_entity.0, v);
        }
        self.current_response = match tab.as_str() {
            "Entities" => entity_view::draw_entities(self, ui, tab),
            "Inspector" => inspector_view::draw_inspector(self, ui, tab),
            "Field" => field_view::draw_field(self, ui, tab),
            "Scene" => scene_window::draw_scene_window(self, ui, tab),
            _unknown_window => {
                log::error!("Window not found! {}", _unknown_window);
                return;
            }
        };
    }

    fn id(&mut self, tab: &mut Self::Tab) -> egui::Id {
        egui::Id::new(self.title(tab).text())
    }

    fn closeable(&mut self, _tab: &mut Self::Tab) -> bool {
        false
    }

    fn context_menu(
        &mut self,
        ui: &mut Ui,
        tab: &mut Self::Tab,
        surface: SurfaceIndex,
        node: egui_dock::NodeIndex,
    ) {
        if ui.label("Refresh").clicked() {
            log::info!("Clicked refresh")
        }

        ui.checkbox(&mut self.debug_mode, "Debug mode");
    }
}

static mut WORLD_REF: Option<*mut World> = None;

pub fn update_inspector<'a>(world: &mut World) {
    world.resource_scope(
        |world_scoped: &mut World, mut editor: Mut<EditorInterface>| {
            unsafe { WORLD_REF = Some(std::ptr::from_mut(world_scoped)) }

            for key in world_scoped
                .resource_mut::<Input>()
                .iter_editor_keys_from_events()
            {
                match key {
                    engine::input::key::keycode_converter::ButtonEvent::Scroll { x, y } => {
                        // TODO: Get system scroll amount and set it here
                        editor.gui.input.mouse_wheel_event(x * 10.0, y * 10.0)
                    }
                    engine::input::key::keycode_converter::ButtonEvent::Text(ch) => {
                        editor.gui.input.text_input_event(ch)
                    }
                }
            }

            let ctx = editor.gui.ctx();

            egui::TopBottomPanel::top("Menu").show(&ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Menu");
                    ui.separator();
                    ui.checkbox(&mut editor.window.debug_mode, "Debeg mode")
                });
            });

            egui::Window::new("Dockable Windows")
                .constrain(true)
                .show(&ctx, |ui| {
                    EditorInterface::inspector_dock_ui(&mut editor, ui, world_scoped);
                });

            ggegui::Gui::update(
                &mut editor.gui,
                world_scoped
                    .resource_mut::<GgezInterface>()
                    .get_context_mut(),
            );

            unsafe { WORLD_REF = None }
        },
    );
}

pub fn draw_editor_gui(editor: Res<EditorInterface>, mut engine: ResMut<GgezInterface>) {
    engine.get_canvas_mut().unwrap().draw(
        &editor.gui,
        DrawParam::default().dest([0.0, 0.0]).z(10000000),
    );
}
