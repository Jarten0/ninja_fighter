pub mod box_collider;
mod convex_mesh;
mod gravity_settings;
pub mod mesh_editor;
pub mod mesh_renderer;
mod traits;

pub use box_collider::BoxCollider;
pub use convex_mesh::ConvexMesh;
use engine::scene::ObjectID;
pub use gravity_settings::GravitySettings;

use bevy_ecs::component::Component;
use bevy_ecs::system::Query;
use bevy_reflect::Reflect;
use engine::space::Position;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Debug;
use traits::SuperMesh;

#[derive(Debug, Component, Default, Reflect)]
pub struct Collider {
    #[reflect(ignore)]
    pub meshes: HashMap<ObjectID, Box<dyn SuperMesh>>,
}

impl Serialize for Collider {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let _s = serializer.serialize_struct("Collider", 1)?;
        todo!()
    }
}

impl Collider {
    pub fn new(meshes: HashMap<ObjectID, Box<dyn SuperMesh>>) -> Self {
        Self { meshes }
    }

    pub fn empty() -> Self {
        Self {
            meshes: HashMap::new(),
        }
    }

    pub fn get_mesh(&self, mesh_id: &ObjectID) -> Option<&Box<dyn SuperMesh>> {
        self.meshes.get(mesh_id)
    }

    pub fn get_mesh_mut(&mut self, mesh_id: &ObjectID) -> Option<&mut Box<dyn SuperMesh>> {
        self.meshes.get_mut(mesh_id)
    }
}

pub fn update(mut query: Query<(&mut Collider, &Position)>) {
    for (mut collider, _position) in query.iter_mut() {
        for _mesh in &mut collider.meshes {}
    }
}
