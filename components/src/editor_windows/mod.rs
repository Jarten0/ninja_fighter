use bevy_ecs::entity::Entity;
use engine::scene::{ObjectID, SceneData};
use engine::GgezInterface;

use crate::collider::mesh_renderer::MeshOverride;
use crate::collider::Collider;

#[derive(Debug, Default)]
pub struct MeshEditorTab {
    focus_state: MeshEditorFocusState,
    entity_list: Option<Vec<Entity>>,
}

#[derive(Debug, Clone)]
enum MeshEditorFocusState {
    None {
        entity_list: Vec<(Entity, usize, String)>,
    },
    Entity {
        entity: Entity,
    },
    Mesh {
        entity: Entity,
        mesh_id: ObjectID,
    },
}

impl Default for MeshEditorFocusState {
    fn default() -> Self {
        Self::None {
            entity_list: Vec::default(),
        }
    }
}

impl engine::editor::EditorTab for MeshEditorTab {
    fn name() -> &'static str
    where
        Self: Sized,
    {
        "MeshEditor"
    }

    fn display_name(&self) -> String {
        "Mesh Editor".to_string()
    }

    fn ui(
        &mut self,
        window_state: &mut engine::editor::WindowState,
        ui: &mut egui::Ui,
    ) -> Option<engine::editor::TabResponse> {
        match &mut self.focus_state {
            MeshEditorFocusState::None { entity_list } => {
                ui.label("No focused entity");

                if entity_list.len() == 0 {
                    ui.label("No entites found with a collider component!");
                } else {
                    for (entity, mesh_count, name) in entity_list.iter() {
                        let small_button = ui
                            .small_button(name.as_str())
                            .on_hover_text(format!("Mesh count: {}", mesh_count));

                        if small_button.clicked() {
                            self.focus_state = MeshEditorFocusState::Entity { entity: *entity };
                            return None;
                        }
                    }
                }

                if ui.button("Refresh").clicked() {
                    for (entity, collider, scene_data) in window_state
                        .world_mut()
                        .query::<(Entity, &Collider, &SceneData)>()
                        .iter(window_state.world_ref())
                    {
                        entity_list.push((
                            entity,
                            collider.meshes.len(),
                            scene_data.entity_name.clone(),
                        ))
                    }
                }
            }
            MeshEditorFocusState::Entity { entity } => {
                let Ok(collider) = window_state
                    .world_mut()
                    .query::<&mut Collider>()
                    .get(window_state.world_mut(), *entity)
                else {
                    ui.label("No collider component found on this entity");
                };

                for (id, mesh) in &mut collider.meshes {
                    match mesh {
                        crate::collider::MeshType::Convex(mesh) => {
                            ui.label(
                                mesh.vertices
                                    .iter()
                                    .map(|vertex| vertex.to_string())
                                    .fold(String::new(), |value| todo!())
                                    .collect::<Vec<String>>(),
                            );
                        }
                    }
                }
            }
            MeshEditorFocusState::Mesh { entity, mesh_id } => todo!(),
        }

        None
    }

    fn draw(&self, canvas: &mut ggez::graphics::Canvas) {}
}
