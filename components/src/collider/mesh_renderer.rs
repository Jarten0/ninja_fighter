use super::traits::RenderableMesh;
use super::{Collider, SuperMesh};
use bevy_ecs::component::Component;
use bevy_ecs::reflect::ReflectComponent;
use bevy_ecs::system::{Query, Res, ResMut};
use bevy_reflect::Reflect;
use engine::scene::ObjectID;
use engine::{Camera, GgezInterface};
use ggez::graphics::{self, *};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// TODO: Rewrite functionality same as above
/// Draws collider vertecies/edges if debug is enabled
pub fn draw(
    query: Query<(&MeshRenderer, &Collider)>,
    mut engine: ResMut<GgezInterface>,
    _camera: Res<Camera>,
) {
    for (renderer, collider) in query.iter() {
        // initial param before applying camera offset, and maybe shaders later
        let initial_param = &renderer.draw_param;

        // dont worry about it for now, just take those initial parameters
        let final_param = initial_param.clone();

        for (_mesh_id, mesh) in &collider.meshes {
            let drawable = match mesh {
                super::MeshType::Convex(convex_mesh) => {
                    convex_mesh.into_graphics_mesh(&engine.get_context().gfx)
                }
            };

            engine
                .get_canvas_mut()
                .expect("ColliderMesh should only be called in a draw schedule")
                .draw(&drawable, final_param);
            // }
        }
    }
}

/// Renders all of the meshes on the current entity.
///
/// If you want to override this functionality, add in a mesh overrider
///
/// ...which has yet to be coded in. //TODO: Do that
#[derive(Debug, Component, Clone, Default, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct MeshRenderer {
    /// The default draw parameters for every mesh. To override this on a per-mesh basis, add a mesh overrider.
    ///
    /// Or just move the mesh to a new entity.
    #[reflect(ignore)]
    #[serde(serialize_with = "engine::render::serialize_draw_param")]
    #[serde(deserialize_with = "engine::render::deserialize_draw_param")]
    pub draw_param: ggez::graphics::DrawParam,

    #[reflect(ignore)]
    #[serde(skip)]
    pub(crate) mesh_overrides: HashMap<ObjectID, MeshOverride>,
}

impl MeshRenderer {
    pub fn new() -> Self {
        let draw_param = DrawParam::new().color(Color::MAGENTA);

        Self {
            draw_param: draw_param,
            mesh_overrides: HashMap::new(),
        }
    }

    pub fn new_with_param(param: DrawParam) -> MeshRenderer {
        Self {
            draw_param: param,
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
