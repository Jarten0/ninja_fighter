use core::fmt;

use bevy_ecs::reflect::ReflectComponent;
use bevy_reflect::Reflect;
use engine::space::Vertex;
use engine::GgezInterface;

use bevy_ecs::component::Component;
use bevy_ecs::system::Query;

use ggez::graphics::GraphicsContext;
use ggez::graphics::Mesh as DrawMesh;
use ggez::graphics::MeshData;
use ggez::graphics::Rect;
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

        collider_mesh.drawable_mesh = Some(DrawMesh::from_data(
            gfx,
            MeshData {
                vertices: &vertices,
                indices: &[],
            },
        ));
    }
}

#[derive(Debug, Component, Clone, Reflect, Default)]
#[reflect(Component)]
pub struct ColliderMesh {
    pub(crate) vertecies_list: Vec<Vertex>,
    #[reflect(ignore)]
    pub(crate) drawable_mesh: Option<DrawMesh>,
    pub(crate) serialized_draw_mesh: Option<SerializedDrawMesh>,
}

#[derive(Debug, Clone, Reflect, Default)]
pub struct SerializedDrawMesh {
    // pub(crate) verts: ArcBuffer,
    // pub(crate) inds: ArcBuffer,
    pub(crate) vertex_count: usize,
    pub(crate) index_count: usize,
    pub(crate) bounds: SerializedRect,
}

impl SerializedDrawMesh {
    fn into_rect(self, gfx: &GraphicsContext) -> DrawMesh {
        DrawMesh::from_data(gfx, raw)
    }
}

#[derive(Debug, Reflect, Clone, Default)]
pub struct SerializedRect {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

impl Into<Rect> for SerializedRect {
    fn into(self) -> Rect {
        Rect {
            x: self.x,
            y: self.y,
            w: self.w,
            h: self.h,
        }
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
            drawable_mesh: Some(drawable_mesh),
        }
    }

    pub(crate) fn get_drawable(&self) -> &Option<DrawMesh> {
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
