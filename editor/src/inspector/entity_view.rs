use crate::inspector::inspector_view::InspectorTab;

use super::EditorTab;
use super::TabResponse;
use super::WindowState;
use bevy_ecs::entity::Entity;
use bevy_ecs::world::Mut;
use bevy_ecs::world::World;
use bevy_reflect::Reflect;
use engine::scene::SceneData;
use engine::scene::SceneManager;
use log::*;
use std::collections::HashMap;

type SortedSceneList = Vec<(String, Entity)>;

/// Displays every entity in the current target scene, as well as entities in other loaded scenes and even orphaned entities not belonging to a scene.
#[derive(Debug, Default)]
pub struct EntityHeirarchyTab {
    /// Used when renaming an entity manually, since changes are not applied until the text edit loses focus
    pub cached_entity_name: String,
    pub scenes: Vec<(String, Entity)>,
}

/// Called GETS for short, because why not ;)
#[derive(Debug, Reflect)]
pub struct GlobalEntityTabState {
    pub scene_list: SortedSceneList,
}

impl EditorTab for EntityHeirarchyTab {
    fn ui(&mut self, window_state: &mut WindowState, ui: &mut egui::Ui) -> Option<TabResponse> {
        if let Some(gets_reflected) = window_state.tab_mut::<GlobalEntityTabState>() {
            let gets = gets_reflected
                .downcast_mut::<GlobalEntityTabState>()
                .expect("expected GETS to be set only by EntityHeirarchyTab");

            let mut scene_list = gets.scene_list.clone();

            window_state
                .world_mut()
                .resource_scope::<SceneManager, ()>(|world, res| {
                    if scene_list.len() != res.current_scenes.len() {
                        scene_list.clear();

                        for (name, entity) in &res.current_scenes {
                            scene_list.push((name.to_owned(), *entity)); //unoptimal lol
                        }

                        scene_list.sort();
                    }
                });

            let gets = window_state
                .tab_mut::<GlobalEntityTabState>()
                .unwrap()
                .downcast_mut::<GlobalEntityTabState>()
                .unwrap();

            if scene_list.len() != gets.scene_list.len() {
                gets.scene_list = scene_list;
            }
        } else {
            let mut scene_list = SortedSceneList::new();

            window_state
                .world_mut()
                .resource_scope::<SceneManager, ()>(|world, res| {});

            window_state.set::<GlobalEntityTabState>(Box::new(GlobalEntityTabState { scene_list }));
        }

        let mut new_entities = HashMap::new();

        for (entity, scene_data) in window_state
            .world_mut()
            .query::<(Entity, &SceneData)>()
            .iter(&window_state.world_ref())
        {
            if !window_state.entities_in_current_scene.contains(&entity) {
                new_entities.insert(entity, scene_data.entity_name.clone());
            }
        }
        for entity in new_entities {
            window_state.entities_in_current_scene.push(entity.0);
        }

        if window_state.entities_in_current_scene.len() == 0 {
            ui.label("There are no entities in the current scene");
        }

        let query = &mut window_state.world_mut().query::<&SceneData>();

        for (index, entity) in window_state
            .entities_in_current_scene
            .clone()
            .into_iter()
            .enumerate()
        {
            let name = match query.get(window_state.world_ref(), entity) {
                Ok(ok) => &ok.entity_name,
                Err(err) => {
                    log::error!("{}", err.to_string());
                    window_state.entities_in_current_scene.remove(index);
                    window_state.components.remove(&entity);

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

        ui.separator();

        let scene_list = window_state
            .tab::<GlobalEntityTabState>()
            .unwrap()
            .downcast_ref::<GlobalEntityTabState>()
            .unwrap()
            .scene_list
            .clone();

        window_state
            .world_mut()
            .resource_scope::<SceneManager, ()>(|world, res| {
                for (name, scene_entity) in scene_list {
                    ui.collapsing(name, |ui| {
                        ui.label("todo lol");
                    });
                }
            });

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
