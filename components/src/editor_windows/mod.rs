use bevy_ecs::entity::Entity;
use engine::scene::{ObjectID, SceneData};
use engine::space::Vertex;
use engine::GgezInterface;

use crate::collider::mesh_renderer::MeshOverride;
use crate::collider::{Collider, ConvexMesh, MeshType};

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
        entity_name: String,
    },
    Mesh {
        entity: Entity,
        entity_name: String,
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
                            self.focus_state = MeshEditorFocusState::Entity {
                                entity: *entity,
                                entity_name: name.to_owned(),
                            };
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
            MeshEditorFocusState::Entity {
                entity,
                entity_name,
            } => {
                let Ok(mut collider) = window_state
                    .world_mut()
                    .query::<&mut Collider>()
                    .get_mut(window_state.world_mut(), *entity)
                else {
                    ui.label("No collider component found on this entity");
                    return None;
                };

                ui.label("Listing meshes belonging to ".to_string() + entity_name);

                for (id, mesh) in &collider.meshes {
                    match mesh {
                        MeshType::Convex(mesh) => {
                            if ui.button("Convex mesh").clicked() {
                                self.focus_state = MeshEditorFocusState::Mesh {
                                    entity: *entity,
                                    mesh_id: mesh.mesh_id,
                                    entity_name: entity_name.to_string(),
                                };
                                return None;
                            }
                            let text = mesh
                                .vertices
                                .iter()
                                .map(|vertex| vertex.to_string())
                                .fold(String::new(), |acc, value| acc + value.as_str() + ", ");

                            ui.label(text);
                        }
                    }
                }
                ui.menu_button("Add mesh", |ui| {
                    if ui.button("Add convex mesh").clicked() {
                        let mesh = ConvexMesh::new(vec![
                            (0.0, 0.0),
                            (20.0, 0.0),
                            (20.0, 20.0),
                            (0.0, 20.0),
                        ]);
                        collider.meshes.insert(mesh.mesh_id, MeshType::Convex(mesh));
                    }
                });
            }
            MeshEditorFocusState::Mesh {
                entity,
                mesh_id,
                entity_name,
            } => {
                let Some(mut collider) = window_state.world_mut().get_mut::<Collider>(*entity)
                else {
                    ui.label("Could not find collider");
                    return None;
                };

                let Some(mesh) = collider.get_mesh_mut(mesh_id) else {
                    ui.label("Could not find mesh");
                    return None;
                };

                ui.label(format!(
                    "Editing mesh {} on entity {}",
                    mesh_id, entity_name
                ));

                match mesh {
                    MeshType::Convex(mesh) => for vertex in &mut mesh.vertices {},
                }

                ui.label("Missing implementation of mesh editor");
            }
        }

        None
    }

    fn draw(&self, canvas: &mut ggez::graphics::Canvas) {}
}
