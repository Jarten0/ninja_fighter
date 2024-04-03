use std::collections::HashMap;

use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::system::{Query, Res, ResMut, Resource};
use bevy_reflect::Reflect;
use engine::scene::ObjectID;
use engine::{space, Camera, GgezInterface};
use ggez::graphics::{self, *};
use serde::{Deserialize, Serialize};

use super::{Collider, SuperMesh};

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
    for (renderer, collider) in query.iter() {
        // initial param before applying camera offset, and maybe shaders later
        let initial_param = match &renderer.draw_param {
            Some(param) => param,
            None => continue,
        };

        // dont worry about it for now, just take those initial parameters
        let final_param = initial_param.clone();

        let mut drawables: Vec<Mesh> = Vec::new();
        let context = engine.get_context();
        for mesh in &collider.meshes {
            let drawable = mesh.drawable(&context.gfx);
            drawables.push(drawable);
        }
        for mesh in drawables {
            engine
                .get_canvas_mut()
                .expect("ColliderMesh should only be called in a draw schedule")
                .draw(&mesh, final_param);
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
    pub(crate) mesh_overrides: HashMap<ObjectID, MeshOverride>,
}

impl Serialize for MeshRenderer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // use serde::ser::SerializeStruct;
        // let s = serializer.serialize_struct("MeshRenderer", 2)?;

        // let drawparams = s.serialize_field("DrawParam", &self.draw_param)?;

        // drawparams.

        todo!()
    }
}

impl MeshRenderer {
    pub fn new() -> Self {
        let draw_param = DrawParam::new().color(Color::MAGENTA);

        Self {
            draw_param: Some(draw_param),
            mesh_overrides: HashMap::new(),
        }
    }

    pub fn new_with_param(param: DrawParam) -> MeshRenderer {
        Self {
            draw_param: Some(param),
            mesh_overrides: HashMap::new(),
        }
    }

    pub fn add_override(&mut self, overrider: MeshOverride) {
        self.mesh_overrides.insert(overrider.mesh_id, overrider);
    }

    pub fn get_override(&self, id: ObjectID) -> Option<&MeshOverride> {
        self.mesh_overrides.get(&id)
    }

    pub fn get_override_mut(&mut self, id: ObjectID) -> Option<&mut MeshOverride> {
        self.mesh_overrides.get_mut(&id)
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

impl From<&dyn SuperMesh> for MeshOverride {
    fn from(value: &dyn SuperMesh) -> Self {
        let draw_vertices = value
            .get_vertices()
            .iter()
            .map(|value| (*value).into())
            .collect::<Vec<graphics::Vertex>>()
            .into();
        let mesh_id = value.get_id();
        Self {
            mesh_id,
            draw_vertices,
            indices: Some(value.build_indices().unwrap()),
            draw_param: Some(DrawParam::new()),
        }
    }
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
