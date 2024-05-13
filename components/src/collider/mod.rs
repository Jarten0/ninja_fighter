pub mod box_collider;
mod convex_mesh;
mod gravity_settings;
pub mod mesh_editor;
pub mod mesh_renderer;
mod traits;

use bevy_ecs::reflect::ReflectComponent;
pub use box_collider::BoxCollider;
pub use convex_mesh::ConvexMesh;

use engine::scene::ObjectID;
use ggez::graphics::{self, Drawable};
pub use gravity_settings::GravitySettings;

use bevy_ecs::component::Component;
use bevy_ecs::system::Query;
use bevy_reflect::Reflect;
use engine::space::Position;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Debug;
use traits::SuperMesh;

/// A container for a set of meshes that are responsible for collision handling.
#[derive(Debug, Component, Default, Reflect)]
#[reflect(Component)]
pub struct Collider {
    #[reflect(ignore)]
    pub meshes: HashMap<ObjectID, MeshType>,
}

impl Collider {
    pub fn new(meshes: HashMap<ObjectID, MeshType>) -> Self {
        Self { meshes }
    }

    pub fn empty() -> Self {
        Self {
            meshes: HashMap::new(),
        }
    }

    pub fn get_mesh(&self, mesh_id: &ObjectID) -> Option<&MeshType> {
        self.meshes.get(mesh_id)
    }

    pub fn get_mesh_mut(&mut self, mesh_id: &ObjectID) -> Option<&mut MeshType> {
        self.meshes.get_mut(mesh_id)
    }
}

pub fn update(mut query: Query<(&mut Collider, &Position)>) {
    for (mut collider, _position) in query.iter_mut() {
        for _mesh in &mut collider.meshes {}
    }
}

#[derive(Debug, Clone, Reflect)]
pub enum MeshType {
    Convex(ConvexMesh),
}

impl Default for MeshType {
    fn default() -> Self {
        Self::Convex(ConvexMesh::default())
    }
}
