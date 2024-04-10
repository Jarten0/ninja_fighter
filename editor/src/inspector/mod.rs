use bevy_ecs::prelude::*;
use bevy_reflect::{Map, Reflect};
use egui::{Align2, Ui};
use egui_dock::{DockArea, DockState, Style};
use engine::{
    scene::{ComponentInstanceID, SceneData},
    GgezInterface,
};
use ggez::graphics::{self, DrawParam};
use log::*;
use std::collections::HashMap;

type Response = TabResponse;

#[derive(Debug)]
enum TabResponse {
    SwitchToTab(EditorTabTypes),
}

struct MyTabViewer {
    current_response: Option<Response>,
    entities: HashMap<String, Entity>,
    components: HashMap<Entity, HashMap<ComponentInstanceID, String>>,
    focused_entity: Option<Entity>,
}

impl MyTabViewer {
    pub fn new(
        query: &mut Query<(Entity, &SceneData)>,
        mut focused_entity: Option<Entity>,
    ) -> Self {
        let mut components = HashMap::new();
        let mut entities = HashMap::new();

        if let Some(some) = focused_entity {
            if query.get(some).is_err() {
                focused_entity = None;
            }
        }

        for (entity, scene_data) in query.iter() {
            components.insert(entity, scene_data.component_paths.clone());
            entities.insert(scene_data.object_name.clone(), entity);
        }

        Self {
            entities,
            components,
            focused_entity,
            current_response: None,
        }
    }

    pub fn draw_entities(
        &mut self,
        ui: &mut egui::Ui,
        tab: &mut <Self as egui_dock::TabViewer>::Tab,
    ) -> Option<Response> {
        if self.entities.len() == 0 {
            ui.label("No entities found!");
        }
        for (name, entity) in &self.entities {
            if ui.small_button(name).is_pointer_button_down_on() {
                self.focused_entity = Some(entity.clone());
                return Some(Response::SwitchToTab(EditorTabTypes::Inspector));
            }
        }
        todo!()
    }

    pub fn draw_inspector(
        &mut self,
        ui: &mut egui::Ui,
        tab: &mut <Self as egui_dock::TabViewer>::Tab,
    ) -> Option<Response> {
        if self.focused_entity.is_none() {
            self.focused_entity = self.entities.values().next().cloned();
            if self.focused_entity.is_none() {
                ui.label("No entity in focus");
                return None;
            }
        }
        for (id, component) in self.components.get(&self.focused_entity.unwrap()).unwrap() {
            ui.add(egui::widgets::Button::new(component));
        }
        todo!()
    }

    pub fn draw_field(
        &mut self,
        ui: &mut egui::Ui,
        tab: &mut <Self as egui_dock::TabViewer>::Tab,
    ) -> Option<Response> {
        ui.label("No implementation of field editor at the moment");
        None
    }
}

#[derive(Debug, PartialEq, Eq)]
enum EditorTabTypes {
    Entities,
    Inspector,
    Field,
}

impl ToString for EditorTabTypes {
    fn to_string(&self) -> String {
        match self {
            EditorTabTypes::Entities => "Entities",
            EditorTabTypes::Inspector => "Inspector",
            EditorTabTypes::Field => "Field",
        }
        .to_string()
    }
}

/// Contains the UI functionality of the inspector.
impl egui_dock::TabViewer for MyTabViewer {
    type Tab = EditorTabTypes;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.to_string().into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        self.current_response = match tab {
            EditorTabTypes::Entities => self.draw_entities(ui, tab),
            EditorTabTypes::Inspector => self.draw_inspector(ui, tab),
            EditorTabTypes::Field => self.draw_field(ui, tab),
        };
    }

    fn id(&mut self, tab: &mut Self::Tab) -> egui::Id {
        egui::Id::new(self.title(tab).text())
    }

    fn on_tab_button(&mut self, _tab: &mut Self::Tab, _response: &egui::Response) {}

    fn closeable(&mut self, _tab: &mut Self::Tab) -> bool {
        false
    }
}

#[derive(Resource)]
pub struct EditorInterface {
    gui: ggegui::Gui,
    dock_state: egui_dock::DockState<EditorTabTypes>,
    focused_entity: Option<Entity>,
    current_response: Option<Response>,
    indexs: HashMap<EditorTabTypes, >
}

impl EditorInterface {
    pub fn new(ctx: &mut ggez::Context) -> Self {
        let tabs = [
            EditorTabTypes::Entities,
            EditorTabTypes::Inspector,
            EditorTabTypes::Field,
        ]

        let dock_state = DockState::new(Vec::new());
        let indexs = HashMap::new();

        for tab in tabs {
            dock_state.
        }

        Self {
            gui: ggegui::Gui::new(ctx),
            dock_state,
            focused_entity: None,
            current_response: None,
            indexs: HashMap::new(),
        }
    }

    fn inspector_dock_ui(&mut self, ui: &mut Ui, query: &mut Query<(Entity, &SceneData)>) {
        let mut my_tab_viewer = MyTabViewer::new(query, self.focused_entity);
        DockArea::new(&mut self.dock_state)
            .style(Style::from_egui(ui.style().as_ref()))
            .show_inside(ui, &mut my_tab_viewer);

        if let Some(response) = self.current_response {
            match response {
                TabResponse::SwitchToTab(new_tab) => self.dock_state.set_active_tab(
                    self.dock_state.,
                ),
                _ => unimplemented!(),
            }
        }

        self.current_response = my_tab_viewer.current_response;
    }
}

pub fn update_inspector(
    mut query: Query<(Entity, &SceneData)>,
    mut editor: ResMut<EditorInterface>,
    mut engine: ResMut<GgezInterface>,
) {
    let ctx = engine.get_context_mut();
    let gui_ctx = editor.gui.ctx();

    let show = egui::Window::new("Inspector")
        // .anchor(Align2::RIGHT_TOP, [-23.0, 0.0])
        .constrain(true)
        .show(&gui_ctx, |ui| {
            editor.inspector_dock_ui(ui, &mut query);
        });

    editor.gui.update(engine.get_context_mut());
}

pub fn draw_editor_gui(editor: Res<EditorInterface>, mut engine: ResMut<GgezInterface>) {
    engine.get_canvas_mut().unwrap().draw(
        &editor.gui,
        DrawParam::default().dest([0.0, 0.0]).z(10000000),
    );
    // trace!("Drew gui")
}
