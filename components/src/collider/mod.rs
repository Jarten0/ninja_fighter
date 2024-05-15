pub mod box_collider;
mod convex_mesh;
mod gravity_settings;
pub mod mesh_editor;
pub mod mesh_renderer;
mod traits;

use bevy_ecs::reflect::{self, ReflectComponent, ReflectFromWorld};
pub use box_collider::BoxCollider;
pub use convex_mesh::ConvexMesh;

use engine::assets::SceneAssetID;
use engine::scene::{ObjectID, Scene};
use ggez::graphics::{self, Drawable};
pub use gravity_settings::GravitySettings;

use bevy_ecs::component::Component;
use bevy_ecs::system::Query;
use bevy_reflect::{FromReflect, Reflect, ReflectSerialize};
use engine::space::Position;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use traits::SuperMesh;

/// A container for a set of meshes that are responsible for collision handling.
#[derive(Debug, Component, Default, Reflect)]
#[reflect(FromWorld)]
#[reflect(Component)]
pub struct Collider {
    pub meshes: HashMap<SceneAssetID, MeshType>,
}

impl Collider {
    pub fn new(meshes: HashMap<SceneAssetID, MeshType>) -> Self {
        Self { meshes }
    }

    pub fn empty() -> Self {
        Self {
            meshes: HashMap::new(),
        }
    }

    pub fn get_mesh(&self, mesh_id: &SceneAssetID) -> Option<&MeshType> {
        self.meshes.get(mesh_id)
    }

    pub fn get_mesh_mut(&mut self, mesh_id: &SceneAssetID) -> Option<&mut MeshType> {
        self.meshes.get_mut(mesh_id)
    }

    /// Adds a mesh from the current scene to the collider.
    pub fn add_existing_mesh(&mut self, scene: &Scene, asset_scene_id: SceneAssetID) {
        let asset = scene.get_asset(&asset_scene_id).unwrap();
        let mesh = asset
            .asset_data
            .downcast_mut::<MeshType>()
            .cloned()
            .unwrap();

        self.meshes.insert(asset.asset_name, MeshType::Convex(mesh));
    }

    /// Takes in mesh data, stores it as an asset and adds it to the collider
    pub fn initialize_mesh(&mut self, scene: &mut Scene, mesh: MeshType) {
        scene.create_asset("Tester collider mesh".to_string(), Box::new(mesh));
        // self.meshes.insert(mesh, v)
    }
}

pub fn update(mut query: Query<(&mut Collider, &Position)>) {
    for (mut collider, _position) in query.iter_mut() {
        for _mesh in &mut collider.meshes {}
    }
}

#[derive(Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Serialize)]
pub enum MeshType {
    Convex(ConvexMesh),
}

impl Default for MeshType {
    fn default() -> Self {
        Self::Convex(ConvexMesh::default())
    }
}
