<<<<<<< Updated upstream
//! Functions:
//!
//! * update - Runs physics updates for collider meshes
//!
//! * draw - Draws collider meshes if is_debug_draw is enabled

#![allow(unused)]

use std::ops::Deref;
use std::ops::Range;

use bevy_ecs::component::Component;
use bevy_ecs::reflect;
use bevy_ecs::reflect::ReflectComponent;
use bevy_ecs::system::Query;
use bevy_ecs::system::Res;
use bevy_ecs::system::ResMut;
=======
>>>>>>> Stashed changes
use bevy_reflect::Reflect;
use engine::space;
use engine::space::Position;
use ggez::graphics;
use serde::Deserialize;
use serde::Serialize;
<<<<<<< Updated upstream

/// Runs physics updates for collider meshes
pub fn update(
    mut query: Query<(&mut ConvexMesh, Option<&Position>)>,
    engine: Res<GgezInterface>,
    input: Res<engine::Input>,
) {
    for (mut convex_mesh, entity_position) in query.iter_mut() {
        let entity_position = match entity_position {
            Some(pos) => pos,
            None => {
                log::error!("Position is required for collider_mesh component to update!");
                continue;
            }
        };

        let translation_amount = *convex_mesh.position.deref(); // - entity_position.deref();
=======
use std::ops::Deref;
/// Check if the mesh can create triangle index patterns, and build them if so.
///
/// For example, if the mesh must be convex, `verify_vertices` would check if each vertex is in a proper location,
/// and `build_indices` would build those indices patterns based on the assumption that the set has been verified.
pub trait CertifiableMesh {
    /// Check if the object can build a set of triangles based on a set of rules that will be used in [`build_indices`]
    fn verify_vertices(&self) -> Result<(), String>;
>>>>>>> Stashed changes

        for vertex in &mut convex_mesh.vertices {
            vertex.translate(&translation_amount);
        }

        if engine.is_debug_draw() {
            let transform_tuple = match convex_mesh.debug_draw_param.unwrap().transform {
                graphics::Transform::Values {
                    dest,
                    rotation,
                    scale,
                    offset,
                } => (dest, rotation, scale, offset),
                graphics::Transform::Matrix(_) => todo!(),
            };
            if let None = convex_mesh.focused_vertex {
                for (index, vertex) in &mut convex_mesh.vertices.iter_mut().enumerate().into_iter()
                {
                    vertex.y += 0.05;
                    let get_mouse_pos = input.get_mouse_pos();
                    let offset =
                        vertex
                            .scaled(&transform_tuple.2.into())
                            .translated(&space::Vector2 {
                                x: transform_tuple.0.x,
                                y: -transform_tuple.0.y,
                            });
                    let inverse_sum = &get_mouse_pos.inverse_sum(offset);
                    if inverse_sum.magnitude() < 50.0 {
                        convex_mesh.focused_vertex = Some(index);
                        break;
                    }
                }
            }

            if input.get_action("dragvertex").unwrap().is_pressed()
                && convex_mesh.focused_vertex.is_some()
            {
                // error!("Clicked");
                let index = convex_mesh.focused_vertex.unwrap();
                let mut set = input
                    .get_mouse_pos()
                    .translated(&-(Into::<space::Vector2>::into(transform_tuple.0)));
                set.x /= transform_tuple.2.x;
                set.y /= transform_tuple.2.y;
                (**convex_mesh.vertices.get_mut(index).unwrap()).set(set);
            } else if let Some(index) = convex_mesh.focused_vertex {
                let vertex = convex_mesh.vertices.get(index).unwrap();
                let get_mouse_pos = input.get_mouse_pos();
                let offset = vertex
                    .scaled(&transform_tuple.2.into())
                    .translated(&space::Vector2 {
                        x: transform_tuple.0.x,
                        y: -transform_tuple.0.y,
                    });
                let inverse_sum = &get_mouse_pos.inverse_sum(offset);
                if inverse_sum.magnitude() > 50.0 {
                    convex_mesh.focused_vertex = None;
                }
            }

            convex_mesh.build_indices();

            // update draw vertices

            let vertices = convex_mesh.vertices.to_owned();
            for (index, vtx) in convex_mesh
                .debug_vertecies
                .iter_mut()
                .enumerate()
                .into_iter()
            {
                let vertex = vertices.get(index).unwrap();
                vtx.position = [vertex.x, vertex.y];
                vtx.color = Color::RED.into();
            }

            if let Some(index) = convex_mesh.focused_vertex {
                convex_mesh.debug_vertecies.get_mut(index).unwrap().color = Color::GREEN.into();
            }

            convex_mesh.debug_drawable_mesh = Some(DrawMesh::from_data(
                &engine.get_context().gfx,
                MeshData {
                    vertices: &convex_mesh.debug_vertecies,
                    indices: &convex_mesh.indices,
                },
            ));

            if input.get_action("debuglog").unwrap().is_just_pressed() {
                dbg!(convex_mesh);
            }
        }
    }
}

/// Draws collider vertecies/edges if debug is enabled
pub fn draw(query: Query<&ConvexMesh>, mut engine: ResMut<GgezInterface>, camera: Res<Camera>) {
    if !engine.is_debug_draw() {
        return;
    }

<<<<<<< Updated upstream
    let canvas = engine
        .get_canvas_mut()
        .expect("ColliderMesh should only be called in a draw schedule");
=======
    /// Returns the mutable list of vertices in the mesh.
    fn get_vertices_mut(&mut self) -> &mut Vec<space::Vertex>;
>>>>>>> Stashed changes

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

        // dont worry about it for now, just take those initial parameters
        let mut final_param = initial_param.clone();

        final_param.color(Color::CYAN);

        canvas.draw(drawable, final_param);
    }
}

/// A mesh that works for collision.
///
/// The entire shape must be convex, i.e. each point must have a line of sight from the origin
///
/// [Demo](game\assets\demos\polygonmesh.png)
#[derive(Debug, Component, Clone, Reflect, Default)]
#[reflect(Component)]
pub struct ConvexMesh {
    pub(crate) position: space::Position,
    /// The vertices used to calculate where the mesh's corners are.
    ///
    /// Uses vector vertices, not graphic vertices.
    pub(crate) vertices: Vec<space::Vertex>,
    // The bottom three fields are updated in `update`, then drawn in `draw`
    #[reflect(ignore)]
    pub(crate) debug_vertecies: Vec<graphics::Vertex>,
    #[reflect(ignore)]
    pub(crate) focused_vertex: Option<usize>,
    #[reflect(ignore)]
    pub(crate) debug_drawable_mesh: Option<DrawMesh>,
    #[reflect(ignore)]
    pub(crate) debug_draw_param: Option<DrawParam>,
    #[reflect(ignore)]
    pub(crate) indices: Vec<u32>,
}

impl ConvexMesh {
    pub fn new_with_drawable(
        gfx: &GraphicsContext,
        ggez_vertices: &[graphics::Vertex],
        indices: &[u32],
    ) -> Self {
        let debug_drawable_mesh = Some({
            let raw = MeshData {
                vertices: ggez_vertices,
                indices,
            };

            DrawMesh::from_data(gfx, raw)
        });

        let draw_param = DrawParam::new().color(Color::MAGENTA);

        let vertices = ggez_vertices
            .iter()
            .map(|value| space::Vertex::from(value))
            .collect::<Vec<space::Vertex>>();

        Self {
            vertices,
            debug_drawable_mesh,
            debug_vertecies: ggez_vertices.to_owned(),
            debug_draw_param: Some(draw_param),
            position: Position::default(),
            indices: indices.to_owned(),
            focused_vertex: None,
        }
    }

    pub fn new(vertices: Vec<space::Vertex>) -> Self {
        let draw_param = DrawParam::new().color(Color::MAGENTA);

        let debug_vertecies = vertices
            .clone()
            .iter()
            .map(|value| (*value).into())
            .collect::<Vec<graphics::Vertex>>();

        let mut convex_mesh = Self {
            debug_drawable_mesh: None,
            debug_vertecies,
            vertices,
            debug_draw_param: Some(draw_param),
            position: Position::default(),
            indices: Vec::new(),
            focused_vertex: None,
        };

        convex_mesh
            .build_indices()
            .map_err(|err| format!("Invalid indices build: {}", err))
            .unwrap();

        convex_mesh
    }

    pub fn get_drawable(&self) -> &Option<DrawMesh> {
        &self.debug_drawable_mesh
    }

    pub fn add_vertex(&mut self, vertex: space::Vertex) {
        self.vertices.push(vertex);
        self.debug_vertecies.push(space::Vertex::into(vertex));
    }

    pub fn add_debug_vertex(&mut self, vertex: graphics::Vertex) {
        self.vertices.push(space::Vertex::from(vertex));
        self.debug_vertecies.push(vertex);
    }

    pub fn pop_vertex(&mut self) {
        self.vertices.pop();
        self.debug_vertecies.pop();
    }

<<<<<<< Updated upstream
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
    pub fn validate_convex(&self) -> Result<(), f32> {
        validate_vertices(&self.vertices)
=======
    /// Runs physics updates for collider meshes
    pub fn update(&mut self, position: &Position) {
        let translation_amount = *self.position.deref() - *position.deref();

        for vertex in &mut self.vertices {
            vertex.translate(&translation_amount);
        }
    }
}

impl CertifiableMesh for ConvexMesh {
    fn verify_vertices(&self) -> Result<(), String> {
        let vec = &self.vertices;
        let len = vec.len();
        if len < 3 {
            return Err(0.0.to_string());
        }
        let origin_vertex = vec[0];
        let mut previous_vertex = vec[1];
        let mut checking_vertex;

        let mut previous_angle = origin_vertex.inverse_sum(*previous_vertex).angle();

        for i in 2..len {
            checking_vertex = vec[i];

            let checking_angle = previous_vertex.inverse_sum(*checking_vertex).angle();
            let angle_difference = checking_angle - previous_angle;

            // if angle_difference > 0.0 {
            //     // TODO: make it less than or equal to when not doing box collider
            //     return Err(angle_difference);
            // }

            previous_angle = checking_angle;
            previous_vertex = checking_vertex;
        }
        Ok(())
>>>>>>> Stashed changes
    }

    /// Builds a vec of indices that correlate to how triangles are connected.
    ///
    /// For more info, check out [demo](game\assets\demos\polygonmesh.png).
    pub fn build_indices(&mut self) -> Result<(), f32> {
        self.validate_convex()?;

        self.indices =
            build_convex_indices(self.vertices.len() as u32, Vec::new()).map_err(|v| v as f32)?;

        Ok(())
    }
}

/// Builds a vec of indices that correlate to how triangles are connected.
///
/// When calling this function, validate that the vertices are valid before attempting to build triangles with it.
/// This code assumes that you've already checked and that each triangle can be constructed from the index.
///
/// For more info, check out [demo](game\assets\demos\polygonmesh.png).
fn build_convex_indices(len: u32, mut vec: Vec<u32>) -> Result<Vec<u32>, u32> {
    for i in 0..len - 2 {
        vec.extend([0, 1 + i, 2 + i]);
    }

<<<<<<< Updated upstream
    Ok(vec)
}
=======
    fn get_vertices_mut(&mut self) -> &mut Vec<space::Vertex> {
        &mut self.vertices
    }
>>>>>>> Stashed changes

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
pub fn validate_vertices(vec: &Vec<space::Vertex>) -> Result<(), f32> {
    let len = vec.len();
    if len < 3 {
        return Err(0.0);
    }
    let origin_vertex = vec[0];
    let mut previous_vertex = vec[1];
    let mut checking_vertex;

    let mut previous_angle = origin_vertex.inverse_sum(*previous_vertex).angle();

    for i in 2..len {
        // trace!("vtx#{}, prev angle: {}", i, previous_angle);
        checking_vertex = vec[i];

        let checking_angle = previous_vertex.inverse_sum(*checking_vertex).angle();
        let angle_difference = checking_angle - previous_angle;
        // assert_eq!(angle_difference, 90.0);

        // if angle_difference > 0.0 {
        //     // TODO: make it less than or equal to when not doing box collider
        //     return Err(angle_difference);
        // }

        previous_angle = checking_angle;
        previous_vertex = checking_vertex;
    }
    Ok(())
}

impl Into<Option<graphics::Mesh>> for ConvexMesh {
    fn into(self) -> Option<graphics::Mesh> {
        self.debug_drawable_mesh
    }
}

impl<'md> From<graphics::MeshData<'md>> for ConvexMesh {
    fn from(value: graphics::MeshData) -> Self {
        let vec = value
            .vertices
            .iter()
            .map(|value| space::Vertex::from(value))
            .collect::<Vec<space::Vertex>>();

        let position = Position::new(0.0, 0.0);

        Self {
            position,
            vertices: vec,
            debug_vertecies: value.vertices.to_vec(),
            debug_drawable_mesh: None,
            debug_draw_param: None,
            indices: value.indices.to_owned(),
            focused_vertex: None,
        }
    }
}

// Serialization

impl Serialize for ConvexMesh {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut serialize_seq = serializer.serialize_struct("ColliderMesh", 3)?;
        serialize_seq.serialize_field("position", &self.position)?;
        serialize_seq.serialize_field("vertices", &self.vertices)?;
        serialize_seq.end()
    }
}

impl<'de> Deserialize<'de> for ConvexMesh {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_struct(
            "ColliderMesh",
            &["position", "vertices"],
            ConvexMeshVisitor,
        )
    }
}

struct ConvexMeshVisitor;

impl<'de> serde::de::Visitor<'de> for ConvexMeshVisitor {
    type Value = ConvexMesh;

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
