use self::entity_view::EntityHeirarchyTab;
use self::field_view::FieldViewState;
use self::inspector_view::InspectorViewState;
use engine::editor::InspectableAsField;

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
use std::ops::DerefMut;

pub mod entity_view;
pub mod field_view;
pub mod inspector_view;
mod scene_window;

#[derive(Debug)]
pub enum TabResponse {
    SwitchToTab(String),
    RemoveComponent(Entity, ComponentId),
}

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
}

impl EditorGUI {
    pub(crate) fn new(ctx: &mut ggez::Context, world: &mut World) -> Self {
        Self {
            gui: ggegui::Gui::new(ctx),
            dock_state: DockState::new(vec![EntityHeirarchyTab::create_tab()]),
            current_response: None,
            window: WindowState::new(world),
        }
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

/// The container for state globally available to every tab.
pub struct WindowState
where
    Self: 'static,
    Self: Sync,
    Self: Send,
{
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

struct FieldInspectorWindow
where
    Self: 'static + Send + Sync,
{
    /// If focused, the focused entity, component and component path, and field path.
    ///
    /// The field path is essentially which part of the component is being isolated.
    /// * If it's focusing the component and not any of its fields in particular, the `Vec` will be empty.
    /// * If it's focusing a field belonging to the component, the `Vec` will contain the field name.
    /// * If it's focusing a field inside of another struct inside of the component, the `Vec` will store the field path of the struct, then that field.
    ///   * Repeat recursively if inspecting a field multiple containers deep.
    pub focused_field: Option<(Entity, (ComponentId, String), Vec<String>)>,
}

/// A container for a tab of any kind, as well as it's name.
///
/// Mostly exists for simplicity's sake, making it so that the window viewer doesn't have to request the name of the tab directly every single time.
pub struct EditorTabState
where
    Self: 'static + Sync + Send,
{
    /// uses dynamic tab state instead of an enum or similar to allow dynamic implementation of new tab types
    state: Box<dyn EditorTab>,
    /// The name of the tab. This is not used for identification, but rather just displaying.
    name: String,
    /// The unique ID of the tab, used to differentiate it from others.
    id: egui::Id,
}

impl PartialEq for EditorTabState {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

pub trait EditorTab
where
    Self: Sync + Send + 'static,
{
    /// Instantiate a new tab, then returning it inside of a box.
    ///
    /// This is done since tabs are operated upon dynamically, and you can't return an unsized object through a trait method.
    fn default_boxed() -> Box<Self>
    where
        Self: Sized;

    /// Returns the name of the tab used for debug and for identification
    fn name() -> String
    where
        Self: Sized;

    /// Returns a name to display the tab with, usually derived from the identification name.
    fn display_name(&self) -> String;

    /// Draws all of the elements of the tab inside of the window.
    ///
    /// This will be where the main UI logic will take place.
    fn draw(&mut self, window_state: &mut WindowState, ui: &mut egui::Ui) -> Option<TabResponse>;

    /// Initializes a container for the tab, along with some state important for tab UI functionality.
    fn create_tab() -> EditorTabState
    where
        Self: Sized + 'static,
    {
        let state = Self::default_boxed();
        let name = Self::name();
        let tab_id = TabID::get_new().into();

        EditorTabState {
            state,
            name,
            id: tab_id,
        }
    }
}

impl WindowState {
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

impl egui_dock::TabViewer for WindowState {
    type Tab = EditorTabState;

    fn title(&mut self, current_tab: &mut Self::Tab) -> egui::WidgetText {
        current_tab.name.to_owned().into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, current_tab: &mut Self::Tab) {
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

        self.current_response = current_tab.state.draw(self, ui);
    }

    fn id(&mut self, tab: &mut Self::Tab) -> egui::Id {
        tab.id
    }

    fn closeable(&mut self, _tab: &mut Self::Tab) -> bool {
        true
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

pub fn update_windows<'a>(world: &mut World) {
    world.resource_scope(
        |world_scoped: &mut World, mut editor_resource: Mut<EditorGUI>| {
            unsafe { WORLD_REF = Some(std::ptr::from_mut(world_scoped)) }

            for key in world_scoped
                .resource_mut::<Input>()
                .iter_editor_keys_from_events()
            {
                match key {
                    engine::input::key::keycode_converter::ButtonEvent::Scroll { x, y } => {
                        // TODO: Get system scroll amount and set it here
                        editor_resource
                            .gui
                            .input
                            .mouse_wheel_event(x * 10.0, y * 10.0) // for some reason, ggez or ggegui is kinda funky with it's scroll events. So this just makes it easier to use.
                                                                   // TODO: Figure out that bug and alter this so this buggy workaround can be removed.
                    }
                    engine::input::key::keycode_converter::ButtonEvent::Text(ch) => {
                        editor_resource.gui.input.text_input_event(ch)
                    }
                }
            }

            let ctx = editor_resource.gui.ctx();

            egui::TopBottomPanel::top("Menu").show(&ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Menu");
                    ui.separator();
                    ui.checkbox(&mut editor_resource.window.debug_mode, "Debug mode");
                    ui.separator();

                    ui.menu_button("Add Window", |ui| {
                        add_tab_label_ui::<EntityHeirarchyTab>(ui, &mut editor_resource);
                    });
                });
            });

            egui::CentralPanel::default().show(&ctx, |ui| {
                let deref_mut = editor_resource.deref_mut(); // change detection tripped here regardless of actual changes since splitting borrows doesn't work with `Mut<EditorGUI>`
                DockArea::new(&mut deref_mut.dock_state).show_inside(ui, &mut deref_mut.window)
            });

            ggegui::Gui::update(
                &mut editor_resource.gui,
                world_scoped
                    .resource_mut::<GgezInterface>()
                    .get_context_mut(),
            );

            unsafe { WORLD_REF = None }
        },
    );
}

/// The UI for showing a selectable label that adds a tab when pressed
fn add_tab_label_ui<T: EditorTab>(ui: &mut Ui, editor_resource: &mut Mut<EditorGUI>) {
    let selectable_label = ui.button(T::name());

    // if selectable_label.clicked() {
    //     editor_resource
    //         .dock_state
    //         .get_surface_mut(SurfaceIndex::main())
    //         .unwrap()
    //         .node_tree_mut()
    //         .unwrap()
    //         .push_to_focused_leaf(T::create_tab());

    //     // editor_resource.dock_state.add_window(vec![T::create_tab()]);

    //     ui.close_menu();
    // }

    let context_menu = selectable_label.context_menu(|ui| {
        let add_as_surface = ui.button("Add to main surface");
        if add_as_surface.clicked() {
            editor_resource
                .dock_state
                .get_surface_mut(SurfaceIndex::main())
                .unwrap()
                .node_tree_mut()
                .unwrap()
                .push_to_focused_leaf(T::create_tab());

            // ui.close_menu();
        }

        let add_as_window = ui.button("Add as window");
        if add_as_window.clicked() {
            editor_resource.dock_state.add_window(vec![T::create_tab()]);

            // ui.close_menu();
        }
    });

    // if let Some(ctxmenu) = context_menu {
    //     if ctxmenu.response.clicked_elsewhere() {
    //         // ui.close_menu();
    //     }
    // }
}

pub fn draw_editor_gui(editor: Res<EditorGUI>, mut engine: ResMut<GgezInterface>) {
    engine
        .get_canvas_mut()
        .unwrap()
        .draw(&editor.gui, DrawParam::default().dest([0.0, 0.0]).z(9999));
}

use engine::scene::Counter;
use engine::scene::IDCounter;
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct TabID(usize);

impl IDCounter for TabID {
    fn get_new() -> TabID {
        pub static STATIC_ID_COUNTER: Counter = Counter::new();
        TabID(STATIC_ID_COUNTER.get())
    }
}

impl Into<egui::Id> for TabID {
    fn into(self) -> egui::Id {
        egui::Id::new(self.0)
    }
}
