use crate::engine::Engine;

use bevy_ecs::component::Component;
use bevy_ecs::system::Query;

use ggez::graphics::GraphicsContext;
use ggez::graphics::Mesh as DrawMesh;
use ggez::graphics::MeshData;
use ggez::graphics::Vertex as DrawVertex;

use super::meta_vertex::MetaVertex;

#[derive(Debug, Component, Clone)]
pub(in crate::components) struct ColliderMesh {
    pub(crate) vertecies_list: MetaVertex,
    pub(crate) drawable_mesh: DrawMesh,
}

pub(in crate::components) fn update(
    mut query: Query<&mut ColliderMesh>,
    engine: bevy_ecs::system::Res<Engine>,
) {
    for mut collider_mesh in query.iter_mut() {
        let gfx = &engine.get_context().gfx;
        let vertices: Vec<DrawVertex> = collider_mesh
            .vertecies_list
            .collect()
            .iter()
            .map(|vertex| Into::<DrawVertex>::into(vertex.to_owned()))
            .collect();

        collider_mesh.drawable_mesh = DrawMesh::from_data(
            gfx,
            MeshData {
                vertices: &vertices,
                indices: &[],
            },
        );
    }
}

impl ColliderMesh {
    pub(in crate::components) fn new(gfx: &GraphicsContext) -> Self {
        let drawable_mesh = {
            let raw = MeshData {
                vertices: &[],
                indices: &[],
            };

            DrawMesh::from_data(gfx, raw)
        };

        Self {
            vertecies_list: MetaVertex::default(),
            drawable_mesh,
        }
    }

    pub(in crate::components) fn get_drawable(&self) -> &DrawMesh {
        &self.drawable_mesh
    }
}
