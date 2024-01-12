use core::fmt;

use engine::space::Vertex;
use engine::Engine;

use bevy_ecs::component::Component;
use bevy_ecs::system::Query;

use ggez::graphics::GraphicsContext;
use ggez::graphics::Mesh as DrawMesh;
use ggez::graphics::MeshData;
use ggez::graphics::Vertex as DrawVertex;

#[derive(Debug, Component, Clone)]
pub struct ColliderMesh {
    pub(crate) vertecies_list: Vec<Vertex>,
    pub(crate) drawable_mesh: DrawMesh,
}

pub fn update(mut query: Query<&mut ColliderMesh>, engine: bevy_ecs::system::Res<Engine>) {
    for mut collider_mesh in query.iter_mut() {
        let gfx = &engine.get_context().gfx;
        let vertices: Vec<DrawVertex> = collider_mesh
            .vertecies_list
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
    pub(crate) fn new(gfx: &GraphicsContext) -> Self {
        let drawable_mesh = {
            let raw = MeshData {
                vertices: &[],
                indices: &[],
            };

            DrawMesh::from_data(gfx, raw)
        };

        Self {
            vertecies_list: Vec::default(),
            drawable_mesh,
        }
    }

    pub(crate) fn get_drawable(&self) -> &DrawMesh {
        &self.drawable_mesh
    }
}

impl fmt::Display for ColliderMesh {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.vertecies_list)
    }
}
