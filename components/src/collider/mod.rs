mod box_collider;
mod convex_mesh;
mod gravity_settings;
mod mesh_renderer;
mod traits;

pub use box_collider::BoxCollider;
pub use convex_mesh::ConvexMesh;
pub use gravity_settings::GravitySettings;

use bevy_ecs::component::Component;
use bevy_ecs::system::Query;
use bevy_reflect::Reflect;
use engine::space::Position;
use serde::Serialize;
use std::fmt::Debug;

use traits::SuperMesh;

#[derive(Debug, Component, Default, Reflect)]
pub struct Collider {
    #[reflect(ignore)]
    pub meshes: Vec<Box<dyn SuperMesh>>,
}

impl Serialize for Collider {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = serializer.serialize_struct("Collider", 1)?;
        todo!()
    }
}

impl Collider {
    pub fn new(meshes: Vec<Box<dyn SuperMesh>>) -> Self {
        Self { meshes }
    }

    pub fn empty() -> Self {
        Self { meshes: Vec::new() }
    }
}

pub fn update(mut query: Query<(&mut Collider, &Position)>) {
    for (mut collider, position) in query.iter_mut() {
        for mesh in &mut collider.meshes {}
    }
}
