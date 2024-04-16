use self::entity_view::EntityViewState;
use self::field_view::{FieldViewState, InspectableAsField, InspectorComponentField};
use self::inspector_view::InspectorViewState;
use self::modname::InspectorData;
use bevy_ecs::component::ComponentId;
use bevy_ecs::prelude::*;
use bevy_reflect::{reflect_trait, Reflect, TypeRegistry};
use egui::{Pos2, Ui};
use egui_dock::{DockArea, DockState, Style, SurfaceIndex};
use engine::scene::{SceneData, SceneManager, TestSuperTrait};
use engine::GgezInterface;
use ggez::graphics::DrawParam;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt::Debug;

pub mod entity_view;
pub mod field_view;
pub mod inspector_view;

type Response = TabResponse;

#[derive(Debug, PartialEq, Eq, Hash)]
enum EditorTabTypes {
    Entities,
    Inspector { adding_component: bool },
    Field,
}

impl ToString for EditorTabTypes {
    fn to_string(&self) -> String {
        match self {
            EditorTabTypes::Entities => "Entities",
            EditorTabTypes::Inspector { .. } => "Inspector",
            EditorTabTypes::Field => "Field",
        }
        .to_string()
    }
}

#[derive(Debug)]
enum TabResponse {
    SwitchToTab(EditorTabTypes),
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
    dock_state: egui_dock::DockState<EditorTabTypes>,
    /// (`ID`, `Name`)
    ///
    /// `String` = entity name
    focused_entity: Option<(Entity, String)>,
    current_response: Option<Response>,
    focused_component: Option<ComponentId>,
    window: InspectorWindow,
}

impl EditorInterface {
    pub fn new(ctx: &mut ggez::Context, world: &mut World) -> Self {
        let tabs = vec![
            EditorTabTypes::Entities,
            EditorTabTypes::Inspector {
                adding_component: false,
            },
            EditorTabTypes::Field,
        ];

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
    entities: HashMap<String, Entity>,
    components: HashMap<Entity, Vec<ComponentId>>,
    current_response: Option<Response>,

    /// (`ID`, `Name`)
    ///
    /// `String` = scene name
    pub focused_entity: Option<(Entity, String)>,
    pub focused_component: Option<ComponentId>,
}

impl InspectorWindow {
    /// Requires access to InspectorWindow to get world access.
    ///
    /// Only works under specific circumstances.
    // TODO: Describe those circumstances here. Essentially, don't call outside of `draw_inspector()`
    pub(crate) fn world(&mut self) -> &mut World {
        unsafe { &mut *(WORLD_REF.unwrap()) } // You called `world` when the reference wasn't available
    }

    pub fn new(world: &mut World) -> Self {
        let mut entities = HashMap::new();
        let mut components = HashMap::new();

        for (entity, scene_data) in world.query::<(Entity, &SceneData)>().iter(&world) {
            // if scene_data.hide_in_inspector {
            //     continue;
            // }

            entities.insert(scene_data.object_name.clone(), entity);
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

        for entity in entities.values() {
            let bundle = InspectorData::new(world, *entity);
            world.entity_mut(*entity).insert(bundle);
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
        }
    }
}

impl egui_dock::TabViewer for InspectorWindow {
    type Tab = EditorTabTypes;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.to_string().into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        if let Some(focused_entity) = self.focused_entity.clone() {
            let mut v: Vec<TypeId> = Vec::new();
            for (entity, read) in self
                .world()
                .query::<(Entity, &dyn TestSuperTrait)>()
                .iter(self.world())
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
                .map(|component| self.world().components().get_id(*component).unwrap())
                .collect();

            self.components.insert(focused_entity.0, v);
        }
        self.current_response = match tab {
            EditorTabTypes::Entities => entity_view::draw_entities(self, ui, tab),
            EditorTabTypes::Inspector { .. } => inspector_view::draw_inspector(self, ui, tab),
            EditorTabTypes::Field => field_view::draw_field(self, ui, tab),
        };
    }

    fn id(&mut self, tab: &mut Self::Tab) -> egui::Id {
        egui::Id::new(self.title(tab).text())
    }

    fn closeable(&mut self, _tab: &mut Self::Tab) -> bool {
        false
    }
}

mod modname {
    use std::any::Any;
    use std::any::TypeId;

    use bevy_ecs::prelude::*;
    use engine::scene::TestSuperTrait;

    use engine::scene::SceneManager;

    use std::collections::HashMap;

    use super::field_view::InspectableAsField;
    use super::field_view::InspectorComponentField;

    /// A component containing data for the inspector to use, stored with each entity and updated whenever that entity is in focus.
    ///
    /// Note that this is one of few components that shouldn't show up in the inspector.
    #[derive(Debug, Component)]
    pub struct InspectorData {
        /// Contains each component, stored via its path, and each field, stored via its name.
        pub component_data: HashMap<String, HashMap<String, InspectorComponentField>>,
    }

    impl InspectorData {
        pub fn new(world: &mut World, entity: Entity) -> InspectorData {
            let mut component_data = HashMap::new();

            world.resource_scope(|world: &mut World, res: Mut<SceneManager>| {
                let mut query = world.query::<&dyn TestSuperTrait>();

                for component in query.get(world, entity).unwrap() {
                    if !component.show_in_inspector() {
                        continue;
                    }

                    let path = component.as_reflect().reflect_type_path().to_owned();

                    let mut component_fields = HashMap::new();

                    let type_info = res
                        .type_registry
                        .get_type_info(component.as_reflect().type_id()) // btw as_reflect() is the stupidest workaround ever but it works! goddamn this issue is stupid
                        .expect(&format!("Expected type info on component {}", path));

                    match type_info {
                        bevy_reflect::TypeInfo::Struct(s) => {
                            for field_name in s.field_names() {
                                let inspect_data: &InspectableAsField = match res
                                    .type_registry
                                    .get_type_data(TypeId::of::<InspectableAsField>())
                                {
                                    Some(field_data) => field_data,
                                    None => {
                                        continue;
                                    }
                                };

                                let field_widget = inspect_data.create_widget();

                                let inspectable_field = InspectorComponentField {
                                    field_inspection_data: inspect_data.to_owned(),
                                    field_name: field_name.to_string(),
                                };

                                component_fields.insert(field_name.to_string(), inspectable_field);
                            }
                        }
                        bevy_reflect::TypeInfo::TupleStruct(ts) => todo!(),
                        bevy_reflect::TypeInfo::Enum(e) => todo!(),
                        _ => unreachable!(), // you implemented reflect on your component incorrectly, will not be implementing functionality for that.
                    }

                    component_data.insert(path, component_fields);
                }
            });

            Self { component_data }
        }
    }
}

static mut WORLD_REF: Option<*mut World> = None;

pub fn update_inspector<'a>(world: &mut World) {
    world.resource_scope(
        |world_scoped: &mut World, mut editor: Mut<EditorInterface>| {
            unsafe { WORLD_REF = Some(std::ptr::from_mut(world_scoped)) }

            egui::Window::new("Inspector")
                .id("InspectorWindow".into())
                .constrain(true)
                .show(&editor.gui.ctx(), |ui| {
                    EditorInterface::inspector_dock_ui(&mut editor, ui, world_scoped);

                    ggegui::Gui::update(
                        &mut editor.gui,
                        world_scoped
                            .resource_mut::<GgezInterface>()
                            .get_context_mut(),
                    );
                });

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
