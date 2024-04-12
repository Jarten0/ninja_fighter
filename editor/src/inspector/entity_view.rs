use std::collections::HashMap;

use super::EditorTabTypes;

use bevy_ecs::entity::Entity;
use engine::scene::SceneData;
use log::*;

use super::Response;

use super::InspectorWindow;

#[derive(Debug, Default)]
pub struct EntityViewState {}

pub fn draw_entities(
    state: &mut InspectorWindow,
    ui: &mut egui::Ui,
    tab: &mut <InspectorWindow as egui_dock::TabViewer>::Tab,
) -> Option<Response> {
    if state.entities.len() == 0 {
        ui.label("No entities found!");
    }
    for (name, entity) in &state.entities {
        if ui.small_button(name).clicked() {
            trace!("Clicked entity");
            state.focused_entity = Some((entity.clone(), name.clone()));
            return Some(Response::SwitchToTab(EditorTabTypes::Inspector {
                adding_component: false,
            }));
        }
    }
    if ui.button("Refresh").clicked() {
        state.entities = {
            let mut vec = HashMap::new();
            // for (entity, scene_data) in state
            //     .query::<(Entity, &SceneData)>()
            //     .iter()
            {
                // vec.insert(scene_data.object_name.clone(), entity);
            }
            vec
        }
    }
    None
}
