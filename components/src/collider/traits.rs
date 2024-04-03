use bevy_reflect::Reflect;
use engine::scene::ObjectID;
use ggez::graphics::{Drawable, GraphicsContext, Mesh};
use serde::Serialize;
use std::fmt::Debug;

/// Check if the mesh can create triangle index patterns, and build them if so.
///
/// For example, if the mesh must be convex, `verify_vertices` would check if each vertex is in a proper location,
/// and `build_indices` would build those indices patterns based on the assumption that the set has been verified.
pub trait CertifiableMesh {
    /// Check if the object can build a set of triangles based on a set of rules that will be used in [`build_indices`]
    fn verify_vertices(&self) -> Result<(), String>;

    fn get_vertices(&self) -> &Vec<engine::space::Vertex>;

    /// Returns the mutable list of vertices in the mesh.
    fn get_vertices_mut(&mut self) -> &mut Vec<engine::space::Vertex>;

    fn build_indices(&self) -> Result<Vec<u32>, String>;

    fn into_graphics_mesh(&self, gfx: &ggez::graphics::GraphicsContext) -> ggez::graphics::Mesh;
}

pub trait Identifiable {
    fn get_id(&self) -> ObjectID;
}

pub trait SuperMesh
where
    Self: CertifiableMesh + Reflect + Debug + Identifiable,
{
    /// Meshes can implement cloning functionality, but because [`Clone::clone`] returns `Self` as owned, it's size varies depending on the type that implements it.
    /// Therefore, [`Clone`] cannot be a trait bound for a trait object. So [`SuperMesh::clone`] returns a fixed size box, pointing to a trait of whatever size.
    ///
    /// The blanket implementation for this works so long as the type implements clone, so enabling that should be helpful.
    fn clone(&self) -> Box<dyn SuperMesh>;

    fn serializable(&self) -> &dyn Reflect;

    fn drawable(&self, gfx: &GraphicsContext) -> Mesh;
}

impl<T> SuperMesh for T
where
    T: CertifiableMesh + Reflect + Debug + Clone + Serialize + Identifiable,
{
    fn clone(&self) -> Box<dyn SuperMesh> {
        Box::new(self.clone())
    }

    fn serializable(&self) -> &dyn Reflect {
        self.as_reflect()
    }

    fn drawable(&self, gfx: &GraphicsContext) -> Mesh {
        self.into_graphics_mesh(gfx)
    }
}
