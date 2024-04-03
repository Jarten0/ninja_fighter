use super::mesh_renderer::MeshRenderer;
use crate::collider::*;
use bevy_ecs::entity::Entity;
use bevy_ecs::system::{Query, Res, ResMut, Resource};
use bevy_reflect::Reflect;
use engine::scene::ObjectID;
use engine::{space, GgezInterface};
use ggez::graphics;
use ggez::graphics::Color;
use log::trace;
use mesh_editor::mesh_renderer::MeshOverride;

#[derive(Debug, Clone, Resource, Reflect)]
pub struct MeshEditor {
    pub focus: FocusState,
}

#[derive(Debug, Clone, Reflect)]
pub enum FocusState {
    Idle,
    FocusedOnEntity {
        focused_entity: Entity,
    },
    FocusedOnMesh {
        focused_entity: Entity,
        focused_mesh_id: ObjectID,
        focused_vertex_index: Option<usize>,
    },
}

pub fn update_editor(
    mut query: Query<(&mut MeshRenderer, &mut Collider, Entity)>,
    input: Res<engine::Input>,
    mut editor: ResMut<MeshEditor>,
) {
    let get_mouse_pos = input.get_mouse_pos().to_owned();
    // trace!("Running mesh editor update");
    if input.get_action("dragvertex").unwrap().is_just_released() {
        editor.focus = FocusState::Idle;
    }
    for (mut renderer, mut collider, entity) in query.iter_mut() {
        // trace!("Found new renderer + collider");
        match &mut editor.focus {
            FocusState::Idle => {
                for mesh in &collider.meshes {
                    for (index, vertex) in mesh.get_vertices().iter().enumerate().into_iter() {
                        let magnitude = get_mouse_pos.inverse_sum(**vertex).magnitude();
                        log::trace!("distance {}", magnitude);
                        if magnitude < 10.0 {
                            if input.get_action("dragvertex").unwrap().is_pressed() {
                                log::trace!("Hovering over mesh");
                                renderer
                                    .draw_param
                                    .unwrap()
                                    .color(Color::from_rgb(240, 240, 240));
                            } else if input.get_action("dragvertex").unwrap().is_just_released() {
                                log::trace!("Selected mesh");
                                editor.focus = FocusState::FocusedOnMesh {
                                    focused_entity: entity,
                                    focused_mesh_id: mesh.get_id(),
                                    focused_vertex_index: Some(index),
                                };
                                return;
                            };
                            renderer
                                .draw_param
                                .unwrap()
                                .color(Color::from_rgb(40, 0, 0));
                        }
                    }
                }
            }
            FocusState::FocusedOnEntity { focused_entity } => {
                if !(entity == *focused_entity) {
                    renderer
                        .draw_param
                        .unwrap()
                        .color(Color::from_rgb(40, 0, 0));
                    continue;
                }
            }
            FocusState::FocusedOnMesh {
                focused_entity,
                focused_mesh_id,
                mut focused_vertex_index,
            } => {
                trace!(
                    "id: {:?}, index: {:?}",
                    focused_mesh_id,
                    focused_vertex_index
                );
                if !(entity == *focused_entity) {
                    renderer
                        .draw_param
                        .unwrap()
                        .color(Color::from_rgb(40, 0, 0));
                    continue;
                }
                for (index, mesh) in collider.meshes.iter_mut().enumerate().into_iter() {
                    if !(mesh.get_id() == *focused_mesh_id) {
                        renderer
                            .draw_param
                            .unwrap()
                            .color(Color::from_rgb(40, 20, 0));

                        continue;
                    }

                    let focused_vertex_index: &mut Option<usize> = &mut focused_vertex_index;
                    for (index, vertex) in
                        mesh.get_vertices_mut().iter_mut().enumerate().into_iter()
                    {
                        let inverse_sum = &get_mouse_pos.inverse_sum(**vertex);
                        if inverse_sum.magnitude() < 10.0 {
                            *focused_vertex_index = Some(index);
                            break;
                        }
                    }

                    if let Some(index) = *focused_vertex_index {
                        if input.get_action("dragvertex").unwrap().is_pressed() {
                            *mesh.get_vertices_mut().get_mut(index).unwrap() = get_mouse_pos.into();
                        }

                        {
                            match renderer.get_override_mut(*focused_mesh_id) {
                                Some(x) => x,
                                None => {
                                    let v = MeshOverride::from(&**mesh);
                                    renderer.mesh_overrides.insert(*focused_mesh_id, v);
                                    renderer.get_override_mut(*focused_mesh_id).unwrap()
                                }
                            }
                        }
                        .draw_vertices
                        .as_mut()
                        .unwrap()
                        .get_mut(index)
                        .unwrap()
                        .color = Color::GREEN.into();
                    };

                    return;
                }
                log::error!("Could not find mesh for mesh renderer! Failsafing editor");
                editor.focus = FocusState::Idle;
                return;
            }
        }
    }
}
