use std::collections::HashMap;

use bevy_ecs::entity::Entity;
use engine::scene::SceneData;
use log::*;

use super::Response;

use super::InspectorWindow;

#[derive(Debug, Default)]
pub struct EntityViewState {}

pub(super) fn draw_entities(
    state: &mut InspectorWindow,
    ui: &mut egui::Ui,
    tab: &mut <InspectorWindow as egui_dock::TabViewer>::Tab,
) -> Option<Response> {
    if state.entities.len() == 0 {
        ui.label("There are no entities currently (Try refreshing!)");
    }
    for (name, entity) in &state.entities {
        if ui.small_button(name).clicked() {
            trace!("Clicked on entity [{}]", name);
            state.focused_entity = Some((entity.clone(), name.clone()));
            return Some(Response::SwitchToTab("Inspector".to_owned()));
        }
    }
    ui.separator();
    if ui.button("Refresh").clicked() {
        trace!("Clicked Refresh");
        state.entities = {
            let mut vec = HashMap::new();
            for (entity, scene_data) in state
                .world()
                .query::<(Entity, &SceneData)>()
                .iter(&state.world())
            {
                vec.insert(scene_data.object_name.clone(), entity);
            }
            vec
        }
    }
    None
}
