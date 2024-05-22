use bevy_ecs::entity::Entity;
use engine::assets::SceneAssetID;
use engine::scene::{ObjectID, Scene, SceneData, SceneManager};
use engine::space::Vertex;
use engine::GgezInterface;
use ggez::graphics::{self, DrawParam, Drawable};
use log::trace;

use crate::collider::mesh_renderer::MeshOverride;
use crate::collider::{Collider, ConvexMesh, MeshType};

#[derive(Debug, Default)]
pub struct MeshEditorTab {
    focus_state: MeshEditorFocusState,
    entity_list: Option<Vec<Entity>>,
    vertex_mesh: Option<graphics::Mesh>,
}

#[derive(Debug, Clone)]
enum MeshEditorFocusState {
    None {
        entity_list: Vec<(Entity, usize, String)>,
    },
    Entity {
        entity: Entity,
        entity_name: String,
        creating_asset_name: String,
    },
    Mesh {
        entity: Entity,
        entity_name: String,

        mesh_id: SceneAssetID,
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
        if self.vertex_mesh.is_none() {
            self.vertex_mesh = Some({
                // the mesh builder for the vertex renderer
                let mut mesh_builder = graphics::MeshBuilder::new();
                mesh_builder
                    .rectangle(
                        graphics::DrawMode::stroke(10.0),
                        graphics::Rect {
                            x: 0.0,
                            y: 0.0,
                            w: 20.0,
                            h: 20.0,
                        },
                        graphics::Color::RED,
                    )
                    .unwrap();
                graphics::Mesh::from_data(
                    window_state
                        .world_ref()
                        .resource::<GgezInterface>()
                        .get_context(),
                    mesh_builder.build(),
                )
            })
        };

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
                                creating_asset_name: String::new(),
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
                creating_asset_name,
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
                                    mesh_id: *id,
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
                    ui.text_edit_singleline(creating_asset_name);
                    if ui.button("Add convex mesh").clicked() {
                        let mesh = ConvexMesh::new(vec![
                            (0.0, 0.0),
                            (200.0, 0.0),
                            (200.0, 200.0),
                            (0.0, 200.0),
                        ]);

                        let Some(target_scene) = window_state
                            .world_ref()
                            .resource::<SceneManager>()
                            .target_scene
                        else {
                            log::error!("No target scene found to add asset to");
                            return;
                        };

                        let Some(mut scene) =
                            window_state.world_mut().get_mut::<Scene>(target_scene)
                        else {
                            log::error!("Target scene set to entity without scene component!");
                            return;
                        };

                        scene.create_asset(
                            creating_asset_name.to_owned(),
                            Box::new(MeshType::Convex(mesh.clone())),
                        );

                        let id = scene.get_scene_id_from_name(creating_asset_name.as_str());

                        let Ok(mut collider) = window_state
                            .world_mut()
                            .query::<&mut Collider>()
                            .get_mut(window_state.world_mut(), *entity)
                        else {
                            ui.label("No collider component found on this entity");
                            return;
                        };

                        collider.meshes.insert(id, MeshType::Convex(mesh));
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
                    "Editing mesh {:?} on entity {}",
                    mesh_id, entity_name
                ));

                match mesh {
                    MeshType::Convex(mesh) => {
                        convex_vertices_ui(mesh, ui);
                    }
                }

                ui.label("Missing implementation of mesh editor");
            }
        }

        None
    }

    fn draw(&self, window_state: &engine::editor::WindowState, engine: &mut GgezInterface) {
        let MeshEditorFocusState::Mesh {
            entity,
            entity_name,
            mesh_id,
        } = &self.focus_state
        else {
            return;
        };

        let Some(collider) = window_state.world_ref().get::<Collider>(*entity) else {
            return;
        };

        let mesh = collider.meshes.get(mesh_id).unwrap();

        #[allow(irrefutable_let_patterns)]
        // this exists so that refactors with different mesh types don't break it.
        if let MeshType::Convex(mesh) = mesh {
            graphics::Mesh::from_data(
                &engine.get_context().gfx,
                graphics::MeshBuilder::new()
                    .polygon(
                        graphics::DrawMode::stroke(5.0),
                        &mesh
                            .vertices
                            .iter()
                            .map(|vertex: &Vertex| (*vertex).into())
                            .collect::<Vec<mint::Point2<f32>>>(),
                        graphics::Color::RED,
                    )
                    .unwrap()
                    .build(),
            )
            .draw(engine.get_canvas_mut().unwrap(), DrawParam::default());
            for vertex in &mesh.vertices {
                self.vertex_mesh.as_ref().inspect(|vertex_mesh| {
                    engine.get_canvas_mut().unwrap().draw(
                        *vertex_mesh,
                        DrawParam::default().dest(mint::Point2::from([vertex.x, vertex.y])),
                    )
                });
            }
        }
    }
}

fn convex_vertices_ui(mesh: &mut ConvexMesh, ui: &mut egui::Ui) {
    let mut from = None;
    let mut to = None;
    for (vertex_index, vertex) in mesh.vertices.iter_mut().enumerate() {
        let response = ui.horizontal(|ui| {
            let response =
                ui.dnd_drop_zone::<usize>(egui::Frame::default().inner_margin(8.0), |ui| {
                    let response = ui
                        .dnd_drag_source(
                            egui::Id::new((
                                "convex_vertices_mesh_editor_ui",
                                mesh.mesh_id,
                                vertex_index,
                            )),
                            (vertex_index),
                            |ui| {
                                ui.label(vertex_index.to_string());
                            },
                        )
                        .response;

                    if let (Some(pointer), Some(hovered_payload)) = (
                        ui.input(|i| i.pointer.interact_pos()),
                        response.dnd_hover_payload::<usize>(),
                    ) {
                        if let Some(dragged_payload) = response.dnd_release_payload() {
                            from = Some(dragged_payload);
                            to = Some(vertex_index);
                        }
                    }
                });
            ui.label("x");
            ui.add(egui::DragValue::new(&mut vertex.x));
            ui.label("y");
            ui.add(egui::DragValue::new(&mut vertex.y));
            response
        });
    }
    if let (Some(from), Some(mut to)) = (from, to) {
        mesh.vertices.swap(*from, to);
    }
}
