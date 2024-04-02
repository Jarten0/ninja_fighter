use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::system::{Query, Res, ResMut, Resource};
use bevy_reflect::Reflect;
use engine::scene::ObjectID;
use engine::{space, Camera, GgezInterface};
use ggez::graphics::{self, *};
use serde::{Deserialize, Serialize};

use super::{Collider, SuperMesh};

#[derive(Debug, Clone, Resource, Reflect)]
pub struct MeshEditor {
    focus: FocusState,
}

#[derive(Debug, Clone, Reflect)]
pub enum FocusState {
    Idle,
    FocusedOnEntity {
        focused_entity: Entity,
    },
    FocusedOnMesh {
        focused_entity: Entity,
        focused_mesh_index: usize,
        focused_vertex_index: Option<usize>,
    },
}

pub fn update_editor(
    mut query: Query<(&mut MeshRenderer, &mut Collider, Entity)>,
    engine: Res<GgezInterface>,
    input: Res<engine::Input>,
    mut editor: ResMut<MeshEditor>,
) {
    let get_mouse_pos = input.get_mouse_pos().to_owned();
    for (mut renderer, mut collider, entity) in query.iter_mut() {
        match &mut editor.focus {
            FocusState::Idle => {
                todo!()
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
                focused_mesh_index,
                mut focused_vertex_index,
            } => {
                if !(entity == *focused_entity) {
                    renderer
                        .draw_param
                        .unwrap()
                        .color(Color::from_rgb(40, 0, 0));
                    continue;
                }
                for (index, mesh) in collider.meshes.iter_mut().enumerate().into_iter() {
                    if !(index == *focused_mesh_index) {
                        renderer
                            .draw_param
                            .unwrap()
                            .color(Color::from_rgb(40, 20, 0));

                        continue;
                    }

                    happy_path(
                        renderer,
                        mesh,
                        editor,
                        input,
                        &mut focused_vertex_index,
                        get_mouse_pos,
                    );

                    return;
                }
                log::error!("Could not find mesh for mesh renderer! Failsafing editor");
                editor.focus = FocusState::Idle;
                return;
            }
        }
    }
}

fn happy_path(
    mut renderer: bevy_ecs::world::Mut<'_, MeshRenderer>,
    mut mesh: &mut Box<dyn SuperMesh>,
    mut editor: ResMut<MeshEditor>,
    input: Res<'_, engine::Input>,
    focused_vertex_index: &mut Option<usize>,
    get_mouse_pos: space::Vector2,
) {
    for (index, vertex) in mesh.get_vertices_mut().iter_mut().enumerate().into_iter() {
        let inverse_sum = &get_mouse_pos.inverse_sum(**vertex);
        if inverse_sum.magnitude() < 50.0 {
            *focused_vertex_index = Some(index);
            break;
        }
    }

    if let Some(index) = *focused_vertex_index {
        if input.get_action("dragvertex").unwrap().is_pressed() {
            *mesh.get_vertices_mut().get_mut(index).unwrap() = get_mouse_pos.into();
        }

        // renderer.get.get_mut(index).unwrap().color = Color::GREEN.into();
    }
}

// TODO: Rewrite functionality when meshes are stored in global resources
// pub fn update(
//     query: Query<(&mut MeshRenderer, &Collider)>,
//     engine: Res<GgezInterface>,
//     input: Res<engine::Input>,
//     editor: Res<MeshEditor>,
// ) {
//     for (mut renderer, collider) in query.iter_mut() {
//         // Each draw vertex *must* be at the same coordinates as the mesh's corresponding vertex
//         for index in 0..renderer.debug_vertecies.len() {
//             renderer.debug_vertecies[index].position =
//                 (*collider.get_vertex(index).unwrap().clone()).into();
//         }
//
//         collider.build_indices();
//
//         // update draw vertices
//
//         let vertices = collider.vertices.to_owned();
//         for (index, vtx) in renderer.debug_vertecies.iter_mut().enumerate().into_iter() {
//             let vertex = vertices.get(index).unwrap();
//             vtx.position = [vertex.x, vertex.y];
//             vtx.color = Color::RED.into();
//         }
//
//         if let Some(index) = renderer.focused_vertex {
//             renderer.debug_vertecies.get_mut(index).unwrap().color = Color::GREEN.into();
//         }
//
//         renderer.draw_mesh = Some(Mesh::from_data(
//             &engine.get_context().gfx,
//             MeshData {
//                 vertices: &renderer.debug_vertecies,
//                 indices: &renderer.indices,
//             },
//         ));
//
//         if input.get_action("debuglog").unwrap().is_just_pressed() {
//             dbg!(collider);
//         }
//     }
// }

// TODO: Rewrite functionality same as above
/// Draws collider vertecies/edges if debug is enabled
pub fn draw(
    query: Query<(&MeshRenderer, &Collider)>,
    mut engine: ResMut<GgezInterface>,
    camera: Res<Camera>,
) {
    let canvas = engine
        .get_canvas_mut()
        .expect("ColliderMesh should only be called in a draw schedule");

    for (renderer, collider) in query.iter() {
        // initial param before applying camera offset, and maybe shaders later
        let initial_param = match &renderer.draw_param {
            Some(param) => param,
            None => continue,
        };

        // dont worry about it for now, just take those initial parameters
        let final_param = initial_param.clone();

        for mesh in &collider.meshes {
            // canvas.draw(&mesh, final_param.color(Color::CYAN));
        }
    }
}

/// Renders all of the meshes on the current entity.
///
/// If you want to override this functionality, add in a mesh overrider
///
/// ...which has yet to be coded in. //TODO: Do that
#[derive(Debug, Component, Clone, Default, Reflect)]
pub struct MeshRenderer {
    /// The default draw parameters for every mesh. To override this on a per-mesh basis, add a mesh overrider.
    ///
    /// Or just move the mesh to a new entity.
    #[reflect(ignore)]
    pub(crate) draw_param: Option<DrawParam>,

    #[reflect(ignore)]
    pub(crate) mesh_overrides: Vec<MeshOverride>,
}

impl Serialize for MeshRenderer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_struct("MeshRenderer", 2)?;
        todo!()
    }
}

impl MeshRenderer {
    pub fn new() -> Self {
        let draw_param = DrawParam::new().color(Color::MAGENTA);

        Self {
            draw_param: Some(draw_param),
            mesh_overrides: Vec::new(),
        }
    }

    pub fn new_with_param(param: DrawParam) -> MeshRenderer {
        Self {
            draw_param: Some(param),
            mesh_overrides: Vec::new(),
        }
    }

    pub fn add_override(&mut self, overrider: MeshOverride) {
        self.mesh_overrides.push(overrider);
    }
}

// TODO: Implement Reflect manually since graphics::Vertex cant implement Reflect. Same with Drawparam :\
// TODO: Polish and flesh out functionality
#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct MeshOverride {
    pub mesh_id: ObjectID,
    #[reflect(ignore)]
    pub draw_vertices: Option<Vec<graphics::Vertex>>,
    pub indices: Option<Vec<u32>>,
    #[reflect(ignore)]
    pub draw_param: Option<DrawParam>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(remote = "ggez::graphics::Vertex")]
pub struct ReflectableVertex {
    pub position: [f32; 2],
    pub uv: [f32; 2],
    pub color: [f32; 4],
}

bevy_reflect::impl_reflect!(
    #[type_path = "Vertex"]
    pub struct ReflectableVertex {
        pub position: [f32; 2],
        pub uv: [f32; 2],
        pub color: [f32; 4],
    }
);
