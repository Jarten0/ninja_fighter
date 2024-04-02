use std::fmt::Debug;

use bevy_reflect::Reflect;

use super::convex_mesh::RenderableMesh;

use super::convex_mesh::CertifiableMesh;

/// This component can contain mesh data,
#[bevy_trait_query::queryable]
pub trait MeshContainer {
    fn get_mesh(&self, index: u32) -> &Box<dyn SuperMesh>;
}

pub trait SuperMesh
where
    Self: CertifiableMesh + RenderableMesh + Reflect + Debug,
{
    /// Meshes can implement cloning functionality, but because [`Clone::clone`] returns `Self` as owned, it's size varies depending on the type that implements it.
    /// Therefore, [`Clone`] cannot be a trait bound for a trait object. So [`SuperMesh::clone`] returns a fixed size box, pointing to a trait of whatever size.
    ///
    /// The blanket implementation for this works so long as the type implements clone, so enabling that should be helpful.
    fn clone(&self) -> Box<dyn SuperMesh>;
}

impl<T> SuperMesh for T
where
    T: CertifiableMesh + RenderableMesh + Reflect + Debug + Clone,
{
    fn clone(&self) -> Box<dyn SuperMesh> {
        Box::new(self.clone())
    }
}
