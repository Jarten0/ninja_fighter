use std::collections::HashMap;

use bevy_ecs::entity::Entity;
use bevy_ecs::world::Mut;
use bevy_ecs::world::World;
use engine::scene::SceneData;
use engine::scene::SceneManager;
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
    };

    if state.entities.len() == 0 {
        ui.label("There are no entities currently (Try refreshing!)");
    }
    for (name, entity) in state.entities.clone() {
        let response = ui.small_button(name.clone());

        if response.clicked() {
            trace!("Clicked on entity [{}]", name);
            state.focused_entity = Some((entity.clone(), name.clone()));
            return Some(Response::SwitchToTab("Inspector".to_owned()));
        }

        response.context_menu(|ui: &mut egui::Ui| {
            ui.add(egui::Label::new("Name"));
            ui.text_edit_singleline(
                &mut state
                    .world()
                    .get_mut::<SceneData>(entity)
                    .unwrap()
                    .object_name,
            );
        });
    }

    ui.separator();

    if ui.button("Add entity").clicked() {
        trace!("Adding new entity");
        state
            .world()
            .resource_scope(|world: &mut World, mut res: Mut<SceneManager>| {
                if let Err(err) = res.new_entity(world, "New entity".to_string()) {
                    error!("Could not add entity! [{}]", err.to_string())
                }
            });
    }

    None
}
