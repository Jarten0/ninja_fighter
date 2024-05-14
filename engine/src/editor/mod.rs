//! Traits and structs that need to be available to both the editor and components.
//!
//! The dependency structure doesn't allow circular imports, so these are placed up higher in the dependency heirarchy
//! so as to ensure that all required use cases of them are reachable.

use crate::scene::Counter;
use crate::scene::IDCounter;
use crate::scene::SceneManager;
use crate::space::Vector2;
use bevy_ecs::component::ComponentId;
use bevy_ecs::prelude::*;
use bevy_reflect::{FromType, Reflect};
use egui::{Ui, Widget};
use egui_dock::SurfaceIndex;
use ggez::graphics;
use std::any::Any;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::{Deref, DerefMut};

/// A simple supertrait for `egui::Widget` that requires the type to implement `Sync` and `Send` (also `Debug`)
// #[bevy_reflect::reflect_trait]
#[cfg(feature = "editor_features")]
pub trait FieldWidget: Send + Sync + Sized {
    fn ui(value: &mut dyn Reflect, ui: &mut egui::Ui) {
        // let field_value = value.downcast_mut::<Self>().unwrap(); //you can use this if your type implements reflect

        ui.label("Default implementation of widget for ".to_owned() + value.reflect_type_path());
    }
}

impl FieldWidget for f32 {
    fn ui(value: &mut dyn Reflect, ui: &mut Ui) {
        let value = value.downcast_mut::<f32>().unwrap();

        let response = egui::DragValue::new(value).speed(0.05).ui(ui);

        response.context_menu(|ui| {
            ui.button("hehe :3");
        });
    }
}

impl FieldWidget for f64 {
    fn ui(value: &mut dyn Reflect, ui: &mut Ui) {
        let value = value.downcast_mut::<f64>().unwrap();

        ui.add(egui::Slider::new(value, -100.0..=100.0));
    }
}

impl FieldWidget for bool {
    fn ui(value: &mut dyn Reflect, ui: &mut Ui) {
        let value = value.downcast_mut::<bool>().unwrap();

        ui.add(egui::Checkbox::without_text(value));
    }
}

impl FieldWidget for String {
    fn ui(value: &mut dyn Reflect, ui: &mut egui::Ui) {
        let field_value = value.downcast_mut::<Self>().unwrap(); //you can use this if your type implements reflect

        ui.add(egui::TextEdit::multiline(field_value));
    }
}

impl FieldWidget for Vector2 {
    fn ui(value: &mut dyn Reflect, ui: &mut Ui) {
        let value = value.downcast_mut::<Vector2>().unwrap();

        ui.horizontal_top(|ui| {
            egui::DragValue::new(&mut value.x).prefix("x: ").ui(ui);
            egui::DragValue::new(&mut value.y).prefix("y: ").ui(ui)
        });
    }
}

/// Insert into the type registry.
///
/// States that the type can be inspected as a field, and when it is, display this widget.
///
/// This also works for structs that contain other inspectable fields.
///
/// You must give it information required to serialize, display and edit it via [`egui`]
#[derive(Debug, Clone)]
pub struct InspectableAsField {
    ui_display_fn: fn(&mut dyn Reflect, &mut Ui),
}

impl<T: FieldWidget> FromType<T> for InspectableAsField {
    fn from_type() -> Self {
        Self::new(<T as FieldWidget>::ui)
    }
}

impl InspectableAsField {
    pub fn new(ui_display_fn: fn(&mut dyn Reflect, &mut Ui)) -> Self {
        Self { ui_display_fn }
    }

    pub fn show(&self, ui: &mut egui::Ui, field: &mut dyn Reflect) {
        (self.ui_display_fn)(field, ui)
    }
}

/// A subtrait for trait objects that need instantiating behaviour, and can afford to clone into the heap rather than to stay on the stack.
///
/// A blanket implementation is used on any type that implements [`Default`] and [`Clone`], so you don't need to worry about implementing it yourself.
pub trait BoxedInstantiate {
    /// Create a default version of this type, and push onto the heap.
    fn default_boxed() -> Box<Self>
    where
        Self: Sized;
}

impl<T> BoxedInstantiate for T
where
    T: Default,
{
    fn default_boxed() -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(T::default())
    }
}

#[derive(Debug)]
pub enum TabResponse {
    SwitchToTab(egui::Id),
    RemoveComponent(Entity, ComponentId),
}

pub trait EditorTab
where
    Self: Sync + Send + 'static + core::fmt::Debug + BoxedInstantiate,
{
    /// Returns the name of the tab used for debug and for identification
    fn name() -> &'static str
    where
        Self: Sized;

    /// Returns a name to display the tab with, usually derived from the identification name.
    fn display_name(&self) -> String;

    /// Draws all of the elements of the tab inside of the window.
    ///
    /// This will be where the main UI logic will take place.
    fn ui(&mut self, window_state: &mut WindowState, ui: &mut egui::Ui) -> Option<TabResponse>;

    /// Initializes a container for the tab, along with some state important for tab UI functionality.
    fn create_tab() -> EditorTabState
    where
        Self: Sized + 'static,
    {
        let state = Self::default_boxed();
        let name = Self::name().to_string();
        let tab_id = TabID::get_new().into();

        EditorTabState {
            state,
            name,
            id: tab_id,
        }
    }

    fn draw(&self, canvas: &mut ggez::graphics::Canvas) {}
}

static mut WORLD_REF: Option<*mut World> = None;

/// Sets the pointer that [`WindowState`] uses to reference the current [`World`].
///
/// This should not be used by the end user outside of building entirely new editor applications.
pub unsafe fn set_world_raw_pointer(world: Option<&mut World>) {
    match world {
        Some(world) => WORLD_REF = Some(std::ptr::from_mut(world)),
        None => WORLD_REF = None,
    }
}

/// The container for state globally available to every tab.
pub struct WindowState
where
    Self: 'static,
    Self: Sync,
    Self: Send,
{
    pub entities: Vec<Entity>,
    pub components: HashMap<Entity, Vec<ComponentId>>,
    pub current_response: Option<TabResponse>,
    pub tab_info: TabInfo,
    pub z: graphics::ZIndex,

    /// (`ID`, `Name`)
    ///
    /// `String` = scene name
    pub focused_entity: Option<(Entity, String)>,
    pub focused_component: Option<ComponentId>,
    pub component_modules: HashMap<String, Vec<((String, String), bevy_reflect::TypeRegistration)>>,
    pub debug_mode: bool,
}

impl WindowState {
    /// Requires access to InspectorWindow to get world access.
    ///
    /// Only works under specific circumstances. Will panic when world access is unavailable.
    // TODO: Describe those circumstances here. Essentially, don't call outside of `ui()`
    pub fn world_ref(&self) -> &World {
        unsafe { &*(WORLD_REF.unwrap()) } // You called `world` when the reference wasn't available
    }

    /// Requires access to InspectorWindow to get world access.
    ///
    /// Only works under specific circumstances. Will panic when world access is unavailable.
    // TODO: Describe those circumstances here. Essentially, don't call outside of `ui()`
    pub fn world_mut(&mut self) -> &mut World {
        unsafe { &mut *(WORLD_REF.unwrap()) } // You called `world` when the reference wasn't available
    }

    pub fn new(world: &mut World) -> Self {
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
            current_response: None,

            focused_entity: None,
            focused_component: None,
            component_modules: modules,

            debug_mode: false,
            tab_info: TabInfo::default(),
            z: 1000,
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
            let mut v: Vec<std::any::TypeId> = Vec::new();
            for (entity, read) in self
                .world_mut()
                .query::<(Entity, &dyn crate::scene::TestSuperTrait)>()
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

        self.current_response = current_tab.state.ui(self, ui);
    }

    fn id(&mut self, tab: &mut Self::Tab) -> egui::Id {
        tab.id
    }

    fn closeable(&mut self, _tab: &mut Self::Tab) -> bool {
        true
    }

    fn context_menu(
        &mut self,
        ui: &mut egui::Ui,
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

/// A container for a tab of any kind, as well as it's name.
///
/// Mostly exists for simplicity's sake, making it so that the window viewer doesn't have to request the name of the tab directly every single time.
pub struct EditorTabState
where
    Self: 'static + Sync + Send + Any,
{
    /// uses dynamic tab state instead of an enum or similar to allow dynamic implementation of new tab types
    pub state: Box<dyn EditorTab>,
    /// The name of the tab. This is not used for identification, but rather just displaying.
    pub name: String,
    /// The unique ID of the tab, used to differentiate it from others.
    pub id: egui::Id,
}

impl PartialEq for EditorTabState {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

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

/// A lightweight resource for retrieving the [`egui::Id`]'s of tabs based on their type without having to manually bother the main dock state yourself.
///
/// Completely type erased functions are coming later.
#[derive(Debug, Default, Resource)]
pub struct TabInfo {
    pub tab_ids: HashSet<egui::Id>,
    /// Stores one [`egui::Id`] of each type of tab, based on whichever one has been focused most recently.
    pub recent_tabs: HashMap<String, egui::Id>,
    pub tab_types: HashMap<egui::Id, String>,
}

impl TabInfo {
    /// Gets the [`egui::Id`] for a tab via its type.
    ///
    /// If multiple exist, the id of the most recently focused tab is returned.
    ///
    /// If no tabs of this type exist, returns [`None`].
    pub fn get_tab<T: EditorTab>(&self) -> Option<egui::Id> {
        self.recent_tabs.get(T::name()).copied()
    }

    /// Returns a [`Vec`] of [`egui::Id`]'s that are of the given type, returning an empty [`Vec`] if none exist.
    pub fn get_tabs<T: EditorTab>(&self) -> Vec<egui::Id> {
        let name = T::name();
        let mut vec = vec![];
        for tab_id in &self.tab_ids {
            let Some(tab_type) = self.tab_types.get(tab_id) else {
                log::error!(
                    "Tab {:?} doesn't have an associated type! (Was it closed?)",
                    tab_id
                );
                continue;
            };

            if tab_type == name {
                vec.push(*tab_id);
            }
        }
        vec
    }

    /// Updates state if the tab is focused
    pub fn tab_focused(&mut self, tab_name: String, tab_id: egui::Id) {
        self.tab_ids.insert(tab_id);
        self.recent_tabs.insert(tab_name.clone(), tab_id); // overwrites whatever tab is of the same type, and has not been focused.
        self.tab_types.insert(tab_id, tab_name);
    }

    /// Removes the given tab. Returns `true` if removed successfully, re
    pub fn remove_tab(&mut self, tab_name: &str, tab_id: egui::Id) -> bool {
        self.tab_ids.remove(&tab_id)
            | self.recent_tabs.remove(tab_name).is_some()
            | self.tab_types.remove(&tab_id).is_some()
    }
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
