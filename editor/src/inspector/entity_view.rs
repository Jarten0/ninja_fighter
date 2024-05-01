use std::collections::HashMap;

use bevy_ecs::entity::Entity;
use bevy_ecs::world::Mut;
use bevy_ecs::world::World;
use engine::scene::SceneData;
use engine::scene::SceneManager;
use log::*;

use super::TabResponse;

use super::InspectorWindow;

#[derive(Debug, Default)]
pub struct EntityViewState {
    /// Used when renaming an entity manually, since changes are not applied until the text edit loses focus
    cached_entity_name: String,
}

pub(super) fn draw_entities(
    state: &mut InspectorWindow,
    ui: &mut egui::Ui,
    tab: &mut <InspectorWindow as egui_dock::TabViewer>::Tab,
) -> Option<TabResponse> {
    let mut new_entities = HashMap::new();
    for (entity, scene_data) in state
        .world_mut()
        .query::<(Entity, &SceneData)>()
        .iter(&state.world_ref())
    {
        if !state.entities.contains(&entity) {
            new_entities.insert(entity, scene_data.object_name.clone());
        }
    }
    for entity in new_entities {
        state.entities.push(entity.0);
    }

    if state.entities.len() == 0 {
        ui.label("There are no entities currently (Try refreshing!)");
    }

    let query = &mut state.world_mut().query::<&SceneData>();

    for entity in state.entities.clone() {
        let name = match query.get(state.world_ref(), entity) {
            Ok(ok) => &ok.object_name,
            Err(err) => {
                log::error!("{}", err.to_string());
                continue;
            }
        };

        let response = ui.small_button(name);

        if response.clicked() {
            trace!("Clicked on entity [{}]", name);
            state.focused_entity = Some((entity, name.to_owned()));
            return Some(TabResponse::SwitchToTab("Inspector".to_owned()));
        }

        response.context_menu(|ui: &mut egui::Ui| {
            ui.add(egui::Label::new("Name"));
            let name_edit = ui.add(
                egui::TextEdit::singleline(&mut state.entity_state.cached_entity_name)
                    .char_limit(60)
                    .clip_text(false)
                    .hint_text("(Set entity name)"),
            );
            if name_edit.gained_focus() {
                state.entity_state.cached_entity_name = state
                    .world_mut()
                    .get_mut::<SceneData>(entity)
                    .unwrap()
                    .object_name
                    .clone()
            }
            if name_edit.lost_focus() {
                state
                    .world_mut()
                    .get_mut::<SceneData>(entity)
                    .unwrap()
                    .object_name = state.entity_state.cached_entity_name.clone();
                state.entity_state.cached_entity_name = String::new();
            }
        });
    }

    ui.separator();

    if ui.button("Add entity").clicked() {
        trace!("Adding new entity");
        state
            .world_mut()
            .resource_scope(|world: &mut World, mut res: Mut<SceneManager>| {
                if let Err(err) = res.new_entity(world, "New entity".to_string()) {
                    error!("Could not add entity! [{}]", err.to_string())
                }
            });
    }

    None
}
