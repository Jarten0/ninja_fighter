use crate::inspector::inspector_view::InspectorTab;

use super::EditorTab;
use super::TabResponse;
use super::WindowState;
use bevy_ecs::entity::Entity;
use bevy_ecs::world::Mut;
use bevy_ecs::world::World;
use engine::scene::SceneData;
use engine::scene::SceneManager;
use log::*;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct EntityHeirarchyTab {
    /// Used when renaming an entity manually, since changes are not applied until the text edit loses focus
    pub cached_entity_name: String,
}

impl EditorTab for EntityHeirarchyTab {
    fn ui(&mut self, window_state: &mut WindowState, ui: &mut egui::Ui) -> Option<TabResponse> {
        let mut new_entities = HashMap::new();

        for (entity, scene_data) in window_state
            .world_mut()
            .query::<(Entity, &SceneData)>()
            .iter(&window_state.world_ref())
        {
            if !window_state.entities.contains(&entity) {
                new_entities.insert(entity, scene_data.entity_name.clone());
            }
        }
        for entity in new_entities {
            window_state.entities.push(entity.0);
        }

        if window_state.entities.len() == 0 {
            ui.label("There are no entities currently (Try refreshing!)");
        }

        let query = &mut window_state.world_mut().query::<&SceneData>();

        for entity in window_state.entities.clone() {
            let name = match query.get(window_state.world_ref(), entity) {
                Ok(ok) => &ok.entity_name,
                Err(err) => {
                    log::error!("{}", err.to_string());
                    continue;
                }
            };

            let response = ui.small_button(name);

            if response.clicked() {
                trace!("Clicked on entity [{}]", name);
                window_state.focused_entity = Some((entity, name.to_owned()));

                let Some(get_tab) = window_state.tab_info.get_tab::<InspectorTab>() else {
                    log::info!("Inspector tab not found!");
                    continue;
                };

                return Some(TabResponse::SwitchToTab(get_tab));
            }

            response.context_menu(|ui: &mut egui::Ui| {
                ui.add(egui::Label::new("Name"));
                let name_edit = ui.add(
                    egui::TextEdit::singleline(&mut self.cached_entity_name)
                        .char_limit(60)
                        .clip_text(false)
                        .hint_text("(Set entity name)"),
                );
                if name_edit.gained_focus() {
                    self.cached_entity_name = window_state
                        .world_mut()
                        .get_mut::<SceneData>(entity)
                        .unwrap()
                        .entity_name
                        .clone()
                }
                if name_edit.lost_focus() {
                    window_state
                        .world_mut()
                        .get_mut::<SceneData>(entity)
                        .unwrap()
                        .entity_name = self.cached_entity_name.clone();
                    self.cached_entity_name = String::new();
                }
            });
        }

        ui.separator();

        if ui.button("Add entity").clicked() {
            trace!("Adding new entity");
            window_state.world_mut().resource_scope(
                |world: &mut World, mut res: Mut<SceneManager>| {
                    if let Err(err) = res.new_entity(world, "New entity".to_string()) {
                        error!("Could not add entity! [{}]", err.to_string())
                    }
                },
            );
        }

        None
    }

    fn name() -> &'static str
    where
        Self: Sized,
    {
        "Entity Heirarchy"
    }

    fn display_name(&self) -> String {
        Self::name().to_string()
    }
}
