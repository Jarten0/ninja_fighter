//! Functions:
//!
//! * update - Runs physics updates for collider meshes
//!
//! * draw - Draws collider meshes if is_debug_draw is enabled

#![allow(unused)]

use std::ops::Deref;
use std::ops::Range;

use bevy_ecs::component::Component;
use bevy_ecs::reflect::ReflectComponent;
use bevy_ecs::system::Query;
use bevy_ecs::system::Res;
use bevy_ecs::system::ResMut;
use bevy_reflect::Reflect;
use engine::space;
use engine::space::Position;
use engine::Camera;
use engine::GgezInterface;
use ggez::context::Has;
use ggez::graphics;
use ggez::graphics::Color;
use ggez::graphics::DrawParam;
use ggez::graphics::FillOptions;
use ggez::graphics::GraphicsContext;
use ggez::graphics::Mesh as DrawMesh;
use ggez::graphics::MeshData;
use ggez::graphics::Rect;
use serde::ser::SerializeStruct;
use serde::Deserialize;
use serde::Serialize;

// use engine::space::Vector2 as eVector2;
// use mint::Vector2 as mVector2;
// use nalgebra::Vector2 as nVector2;

/// Runs physics updates for collider meshes
pub fn update(
    mut query: Query<(&mut ConvexColliderMesh, Option<&Position>)>,
    engine: bevy_ecs::system::Res<GgezInterface>,
) {
    for (mut collider_mesh, entity_position) in query.iter_mut() {
        let entity_position = match entity_position {
            Some(pos) => pos,
            None => {
                log::error!("Position is required for collider_mesh component to update!");
                continue;
            }
        };

        let translation_amount = *collider_mesh.position.deref(); // - entity_position.deref();

        for vertex in &mut collider_mesh.vertecies_list {
            vertex.translate(&translation_amount);
        }

        if engine.is_debug_draw() {
            collider_mesh.debug_drawable_mesh = Some(DrawMesh::from_data(
                &engine.get_context().gfx,
                MeshData {
                    vertices: &collider_mesh.debug_vertecies,
                    indices: &(0..(collider_mesh.debug_vertecies.len() as u32))
                        .collect::<Vec<u32>>(),
                },
            ));
        }
    }
}

/// Draws collider vertecies/edges if debug is enabled
pub fn draw(
    query: Query<&ConvexColliderMesh>,
    mut engine: ResMut<GgezInterface>,
    camera: Res<Camera>,
) {
    if !engine.is_debug_draw() {
        return;
    }

    let canvas = engine
        .get_canvas_mut()
        .expect("ColliderMesh should only be called in a draw schedule");

    for mesh in query.iter() {
        let drawable = match &mesh.debug_drawable_mesh {
            Some(mesh) => mesh,
            None => continue,
        };

        // initial param before applying camera offset, and maybe shaders later
        let initial_param = match &mesh.debug_draw_param {
            Some(param) => param,
            None => continue,
        };

        // let dest = match initial_param.transform {
        //     graphics::Transform::Values {
        //         dest,
        //         rotation: _,
        //         scale: _,
        //         offset: _,
        //     } => dest,
        //     graphics::Transform::Matrix(_matrix) => todo!(), //_matrix,// + camera.position,
        // };

        let final_param = initial_param.clone();

        canvas.draw(drawable, final_param)
    }
    // drawtempmesh(engine);
}

fn drawtempmesh(mut engine: ResMut<GgezInterface>) {
    let mut builder = ggez::graphics::MeshBuilder::new();

    builder.rectangle(
        graphics::DrawMode::Fill(FillOptions::DEFAULT),
        Rect {
            x: 50.0,
            y: 50.0,
            w: 100.0,
            h: 100.0,
        },
        Color::CYAN,
    );

    let meshdata = builder.build();
    // let drawable = graphics::Mesh::from_data(&engine.get_context_mut().gfx, meshdata);

    let mut collider_mesh = ConvexColliderMesh::from(meshdata);

    let from_data = graphics::Mesh::from_data(
        &engine.get_context().gfx,
        dbg!(MeshData {
            vertices: &collider_mesh.debug_vertecies,
            indices: &collider_mesh.indices,
        }),
    );

    collider_mesh.debug_drawable_mesh = Some(from_data);

    let drawable = collider_mesh.get_drawable().as_ref().unwrap();

    engine
        .get_canvas_mut()
        .unwrap()
        .draw(drawable, DrawParam::default())
}

/// A mesh that works for collision.
///
/// The entire shape must be convex, i.e. each point must have a line of sight from the origin
///
/// [Demo](game\assets\demos\polygonmesh.png)
#[derive(Debug, Component, Clone, Reflect, Default)]
#[reflect(Component)]
pub struct ConvexColliderMesh {
    pub(crate) position: space::Position,
    pub(crate) vertecies_list: Vec<space::Vertex>,
    // The bottom three fields are updated in `update`, then drawn in `draw`
    #[reflect(ignore)]
    pub(crate) debug_vertecies: Vec<graphics::Vertex>,
    #[reflect(ignore)]
    pub(crate) debug_drawable_mesh: Option<DrawMesh>,
    #[reflect(ignore)]
    pub(crate) debug_draw_param: Option<DrawParam>,
    #[reflect(ignore)]
    pub(crate) indices: Vec<u32>,
}

impl ConvexColliderMesh {
    pub fn new_with_drawable(
        gfx: &GraphicsContext,
        vertices: &[graphics::Vertex],
        indices: &[u32],
    ) -> Self {
        let debug_drawable_mesh = Some({
            let raw = MeshData { vertices, indices };

            DrawMesh::from_data(gfx, raw)
        });

        let draw_param = DrawParam::new().color(Color::MAGENTA);

        Self {
            vertecies_list: vertices
                .iter()
                .map(|value| space::Vertex::from(value))
                .collect::<Vec<space::Vertex>>(),

            debug_drawable_mesh,
            debug_vertecies: vertices.to_owned(),
            debug_draw_param: Some(draw_param),
            position: Position::default(),
            indices: indices.to_owned(),
        }
    }

    pub fn new(vertices: Vec<space::Vertex>) -> Self {
        let draw_param = DrawParam::new().color(Color::MAGENTA);

        let debug_vertecies;
        debug_vertecies = vertices
            .clone()
            .iter()
            .map(|value| (*value).into())
            .collect::<Vec<graphics::Vertex>>();

        let range: Range<u32> = 0..debug_vertecies.len() as u32;
        Self {
            debug_drawable_mesh: None,
            debug_vertecies,
            vertecies_list: vertices,
            debug_draw_param: Some(draw_param),
            position: Position::default(),
            indices: range.into_iter().collect::<Vec<u32>>(),
        }
    }

    pub fn get_drawable(&self) -> &Option<DrawMesh> {
        &self.debug_drawable_mesh
    }

    pub fn add_vertex(&mut self, vertex: space::Vertex) {
        self.vertecies_list.push(vertex);
        self.debug_vertecies.push(space::Vertex::into(vertex));
    }

    pub fn add_debug_vertex(&mut self, vertex: graphics::Vertex) {
        self.vertecies_list.push(space::Vertex::from(vertex));
        self.debug_vertecies.push(vertex);
    }

    pub fn pop_vertex(&mut self) {
        self.vertecies_list.pop();
        self.debug_vertecies.pop();
    }

    /// Checks to see if every vertex is convex.
    ///
    /// The idea is that it checks the angle between the origin and every vertex, and if the angle goes backwards, the triangle is invalid.
    /// Check out this [demo](game\assets\demos\polygonmesh.png) for more details.
    ///
    /// This makes it easier and more performant to render triangles.
    ///
    /// If the vertex is invalid, returns the index of the vertex that is invalid.
    ///
    /// If there are too few vertecies to make a triangle, returns zero.
    pub fn validate_convex(&self) -> Result<(), u32> {
        let len = self.vertecies_list.len();
        if len < 3 {
            return Err(0);
        }
        let origin_vertex = self.vertecies_list[0];
        let mut previous_vertex = self.vertecies_list[1];
        let mut checking_vertex;
        let mut angle = origin_vertex.inverse_sum(*previous_vertex).angle();

        let mut previous_angle = angle;
        for i in 2..len {
            checking_vertex = self.vertecies_list[i];

            let checking_angle = previous_vertex.inverse_sum(*checking_vertex).angle();
            if checking_angle <= previous_angle {
                return Err(i.try_into().unwrap());
            }

            previous_angle = checking_angle;
            previous_vertex = checking_vertex;
        }

        return Ok(());
    }

    pub fn build_indices(&self) -> Result<Vec<u32>, u32> {
        self.validate_convex()?;

        todo!()
    }
}

impl Into<Option<graphics::Mesh>> for ConvexColliderMesh {
    fn into(self) -> Option<graphics::Mesh> {
        self.debug_drawable_mesh
    }
}

impl<'md> From<graphics::MeshData<'md>> for ConvexColliderMesh {
    fn from(value: graphics::MeshData) -> Self {
        let vec = value
            .vertices
            .iter()
            .map(|value| space::Vertex::from(value))
            .collect::<Vec<space::Vertex>>();

        let position = Position::new(0.0, 0.0);

        Self {
            position,
            vertecies_list: vec,
            debug_vertecies: value.vertices.to_vec(),
            debug_drawable_mesh: None,
            debug_draw_param: None,
            indices: value.indices.to_owned(),
        }
    }
}

impl Serialize for ConvexColliderMesh {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut serialize_seq = serializer.serialize_struct("ColliderMesh", 3)?;
        serialize_seq.serialize_field("position", &self.position)?;
        serialize_seq.serialize_field("vertices", &self.vertecies_list)?;
        serialize_seq.end()
    }
}

impl<'de> Deserialize<'de> for ConvexColliderMesh {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_struct(
            "ColliderMesh",
            &["position", "vertices"],
            ColliderMeshVisitor,
        )
    }
}

struct ColliderMeshVisitor;

impl<'de> serde::de::Visitor<'de> for ColliderMeshVisitor {
    type Value = ConvexColliderMesh;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "Was expecting a collider mesh struct")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        todo!()
    }
}

#[derive(Debug, Clone, Default)]
pub struct SerializedDrawMesh {
    pub vertices: Vec<graphics::Vertex>,
    pub indicies: Vec<u32>,
}

impl SerializedDrawMesh {
    fn into_mesh(self, gfx: &GraphicsContext) -> DrawMesh {
        DrawMesh::from_data(
            gfx,
            MeshData {
                vertices: &self
                    .vertices
                    .iter()
                    .map(|v| Into::<graphics::Vertex>::into(*v))
                    .collect::<Vec<graphics::Vertex>>(),
                indices: &self.indicies,
            },
        )
    }
}

impl Serialize for SerializedDrawMesh {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut serialize_struct = serializer.serialize_struct("DrawMesh", 2)?;

        serialize_struct.serialize_field(
            "vertices",
            &self
                .vertices
                .iter()
                .map(|value| SerializableVertex::from(value))
                .collect::<Vec<SerializableVertex>>(),
        )?;
        serialize_struct.serialize_field("indices", &self.indicies)?;

        todo!();

        serialize_struct.end()
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct SerializableVertex {
    position: [f32; 2],
    uv: [f32; 2],
    color: [f32; 4],
}

impl From<&graphics::Vertex> for SerializableVertex {
    fn from(value: &graphics::Vertex) -> Self {
        SerializableVertex {
            position: value.position,
            uv: value.uv,
            color: value.color,
        }
    }
}
