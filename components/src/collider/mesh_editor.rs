use super::mesh_renderer::MeshRenderer;
use crate::collider::*;
use bevy_ecs::entity::Entity;
use bevy_ecs::system::{Query, Res, ResMut, Resource};
use bevy_reflect::Reflect;
use engine::scene::ObjectID;
use engine::{GgezInterface};
use ggez::graphics::{self, DrawParam, StrokeOptions};
use ggez::graphics::{Color, Rect, Transform};
use log::trace;
use mesh_editor::mesh_renderer::MeshOverride;
use mint::Point2;
use std::collections::HashMap;

#[derive(Debug, Clone, Resource, Reflect, Default)]
pub struct MeshEditor {
    pub focus: FocusState,
    /// Key = mesh, Value = vertex index in mesh vertices
    vertices_to_draw: HashMap<Entity, (ObjectID, usize)>,
}

#[derive(Debug, Clone, Reflect, Default)]
pub enum FocusState {
    #[default]
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
    if let FocusState::FocusedOnMesh {
        focused_entity,
        focused_mesh_id,
        focused_vertex_index: _,
    } = editor.focus
    {
        if !input.get_action("dragvertex").unwrap().is_pressed() {
            editor.focus = FocusState::FocusedOnMesh {
                focused_entity,
                focused_mesh_id,
                focused_vertex_index: None,
            };
        }
    }
    for (mut renderer, mut collider, entity) in query.iter_mut() {
        // trace!("Found new renderer + collider");
        match editor.focus.clone() {
            FocusState::Idle => {
                for (_id, mesh) in &collider.meshes {
                    for (index, vertex) in mesh.get_vertices().iter().enumerate().into_iter() {
                        let magnitude = get_mouse_pos.inverse_sum(**vertex).magnitude();
                        if magnitude < 15.0 {
                            if input.get_action("dragvertex").unwrap().is_pressed() {
                                log::trace!("Hovering over mesh");
                                renderer
                                    .draw_param
                                    .unwrap()
                                    .color(Color::from_rgb(240, 240, 240));
                            }
                            if input.get_action("dragvertex").unwrap().is_just_released() {
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
                if !(entity == focused_entity) {
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
                if !(entity == focused_entity) {
                    renderer
                        .draw_param
                        .unwrap()
                        .color(Color::from_rgb(40, 0, 0));
                    continue;
                }
                for (index, (id, mesh)) in collider.meshes.iter_mut().enumerate().into_iter() {
                    editor.vertices_to_draw.insert(entity, (*id, index));

                    if !(mesh.get_id() == focused_mesh_id) {
                        renderer
                            .draw_param
                            .unwrap()
                            .color(Color::from_rgb(40, 20, 0));

                        continue;
                    }

                    let focused_vertex_index: &mut Option<usize> = &mut focused_vertex_index;

                    if let None = focused_vertex_index {
                        for (index, vertex) in
                            mesh.get_vertices_mut().iter_mut().enumerate().into_iter()
                        {
                            let inverse_sum = &get_mouse_pos.inverse_sum(**vertex);
                            if inverse_sum.magnitude() < 30.0 {
                                *focused_vertex_index = Some(index);
                                break;
                            }
                        }
                    }

                    if let Some(index) = *focused_vertex_index {
                        if input.get_action("dragvertex").unwrap().is_pressed() {
                            *mesh.get_vertices_mut().get_mut(index).unwrap() = get_mouse_pos.into();
                        } else {
                            *focused_vertex_index = None;
                        }

                        {
                            match renderer.get_override_mut(focused_mesh_id) {
                                Some(x) => x,
                                None => {
                                    let v = MeshOverride::from(&**mesh);
                                    renderer.mesh_overrides.insert(focused_mesh_id, v);
                                    renderer.get_override_mut(focused_mesh_id).unwrap()
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

pub fn draw_editor_interface(
    query: Query<&Collider>,
    editor: Res<MeshEditor>,
    mut engine: ResMut<GgezInterface>,
) {
    for (entity, (mesh_id, index)) in editor.vertices_to_draw.iter() {
        let collider = match query.get(*entity) {
            Ok(collider) => collider,
            Err(err) => {
                log::error!("Could not find collider on entity! [{:?}]", (entity, err));
                continue;
            }
        };

        let vertex = collider
            .get_mesh(mesh_id)
            .expect("The collider to have the mesh with this identifier")
            .get_vertices()
            .get(*index)
            .expect("The mesh to have the vertex at this index");

        let stroke_options = StrokeOptions::DEFAULT;

        stroke_options.with_line_width(8.0);

        let drawable = graphics::Mesh::new_circle(
            &engine.get_context().gfx,
            graphics::DrawMode::Stroke(stroke_options),
            Point2 {
                x: vertex.x,
                y: vertex.y,
            },
            30.0,
            1.0,
            Color::RED,
        )
        .unwrap();

        engine.get_canvas_mut().expect("A canvas for draw functionality (did you incorrectly put this in the tick schedule?)"    ).draw(
            &drawable,
            DrawParam {
                src: Rect::default(),
                color: Color::WHITE,
                transform: Transform::default(),
                z: 0,
            },
        );
    }
}
