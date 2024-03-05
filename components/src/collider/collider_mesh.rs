use core::fmt;

use engine::space::Vertex;
use engine::GgezInterface;

use bevy_ecs::component::Component;
use bevy_ecs::system::Query;

use ggez::graphics::GraphicsContext;
use ggez::graphics::Mesh as DrawMesh;
use ggez::graphics::MeshData;
use ggez::graphics::Vertex as DrawVertex;
use serde::ser::SerializeSeq;
use serde::Deserialize;
use serde::Serialize;

pub fn update(mut query: Query<&mut ColliderMesh>, engine: bevy_ecs::system::Res<GgezInterface>) {
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

#[derive(Debug, Component, Clone)]
pub struct ColliderMesh {
    pub(crate) vertecies_list: Vec<Vertex>,
    pub(crate) drawable_mesh: DrawMesh,
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

impl Serialize for ColliderMesh {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut serialize_seq = serializer.serialize_seq(Some(self.vertecies_list.len()))?;
        for vertex in &self.vertecies_list {
            serialize_seq.serialize_element(vertex);
        }
        serialize_seq.end()
    }
}

impl<'de> Deserialize<'de> for ColliderMesh {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self {
            vertecies_list: Vec::new(),
            drawable_mesh: todo!(),
        })
    }
}
