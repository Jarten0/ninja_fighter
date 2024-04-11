use self::entity_view::EntityViewState;
use self::field_view::FieldViewState;
use self::inspector_view::InspectorViewState;
use bevy_ecs::prelude::*;
use egui::Ui;
use egui_dock::{DockArea, DockState, Style, SurfaceIndex};
use engine::scene::{ComponentInstanceID, SceneData};
use engine::GgezInterface;
use ggez::graphics::DrawParam;
use std::collections::HashMap;

mod entity_view;
mod field_view;
mod inspector_view;

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

#[derive(Resource)]
pub struct EditorInterface {
    gui: ggegui::Gui,
    dock_state: egui_dock::DockState<EditorTabTypes>,
    focused_entity: Option<(Entity, String)>,
    current_response: Option<Response>,
    focused_component: Option<String>,
}

impl EditorInterface {
    pub fn new(ctx: &mut ggez::Context) -> Self {
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
        }
    }

    fn inspector_dock_ui(&mut self, ui: &mut Ui, world: &mut World) {
        let mut inspector_window = InspectorWindow::new(world, self.focused_entity.clone());
        DockArea::new(&mut self.dock_state)
            .style(Style::from_egui(ui.style().as_ref()))
            .show_inside(ui, &mut inspector_window);
        self.focused_entity = inspector_window.focused_entity;
        self.focused_component = inspector_window.focused_component;

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
                // TODO: Remove if this project gains significance
                _ => unimplemented!(),
            }
        }

        self.current_response = inspector_window.current_response;
    }
}

#[derive(Default)]
struct InspectorWindow {
    pub entity_state: EntityViewState,
    pub inspector: InspectorViewState,
    pub field_state: FieldViewState,
    // "global" state (available in all inspector views)
    entities: HashMap<String, Entity>,
    components: HashMap<Entity, HashMap<ComponentInstanceID, String>>,
    current_response: Option<Response>,
    /// `String` = scene name
    pub focused_entity: Option<(Entity, String)>,
    pub focused_component: Option<String>,
}

impl InspectorWindow {
    pub fn new(world: &mut World, focused_entity: Option<(Entity, String)>) -> Self {
        let mut components = HashMap::new();
        let mut entities = HashMap::new();

        for (entity, scene_data) in world.query::<(Entity, &SceneData)>().iter(&world) {
            components.insert(entity, scene_data.component_paths.clone());
            entities.insert(scene_data.object_name.clone(), entity);
        }

        Self {
            inspector: InspectorViewState::default(),
            entity_state: EntityViewState::default(),
            field_state: FieldViewState::default(),

            entities,
            components,
            current_response: None,

            focused_entity,
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
        self.current_response = match tab {
            EditorTabTypes::Entities => entity_view::draw_entities(ui, tab),
            EditorTabTypes::Inspector { .. } => inspector_view::draw_inspector(ui, tab),
            EditorTabTypes::Field => field_view::draw_field(ui, tab),
        };
    }

    fn id(&mut self, tab: &mut Self::Tab) -> egui::Id {
        egui::Id::new(self.title(tab).text())
    }

    fn closeable(&mut self, _tab: &mut Self::Tab) -> bool {
        false
    }
}

pub fn update_inspector(world: &mut World) {
    world.resource_scope(|world: &mut World, mut editor: Mut<EditorInterface>| {
        let gui_ctx = editor.gui.ctx();

        let _show = egui::Window::new("Inspector")
            .constrain(true)
            .show(&gui_ctx, |ui| {
                editor.inspector_dock_ui(ui, world);
            });

        editor
            .gui
            .update(world.resource_mut::<GgezInterface>().get_context_mut());
    });
}

pub fn draw_editor_gui(editor: Res<EditorInterface>, mut engine: ResMut<GgezInterface>) {
    engine.get_canvas_mut().unwrap().draw(
        &editor.gui,
        DrawParam::default().dest([0.0, 0.0]).z(10000000),
    );
}
