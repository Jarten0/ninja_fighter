mod box_collider;
mod convex_mesh;
mod gravity_settings;
<<<<<<< Updated upstream
=======
mod mesh_renderer;
mod traits;
>>>>>>> Stashed changes

pub use box_collider::BoxCollider;
pub use convex_mesh::ConvexMesh;
<<<<<<< Updated upstream
pub use convex_mesh::{draw, update};
pub use gravity_settings::GravitySettings;

use bevy_ecs::bundle::Bundle;
use bevy_reflect::Reflect;
use engine::space;
use engine::GgezInterface;
use std::any::Any;

pub trait Collider {
    fn drawable(&self) -> Option<&dyn Any>;
}

#[derive(Debug, Clone, Bundle, Reflect, Default)]
pub struct ColliderBundle {
    gravity: gravity_settings::GravitySettings,
    mesh: convex_mesh::ConvexMesh,
}

impl ColliderBundle {
    pub fn new(engine: &GgezInterface, vertices: &[ggez::graphics::Vertex]) -> Self {
        // let transform = Transform::default();

        let gravity = GravitySettings {
            force: space::DOWN,
            force_cap: 5.0,
        };

        let indices = Vec::new();

        Self {
            gravity,
            mesh: ConvexMesh::new_with_drawable(&engine.get_context().gfx, vertices, &indices),
        }
=======
pub use convex_mesh::RenderableMesh;
pub use gravity_settings::GravitySettings;

use bevy_ecs::component::Component;
use bevy_ecs::system::Query;
use bevy_reflect::Reflect;
use engine::space::Position;
use std::fmt::Debug;

use traits::SuperMesh;

#[derive(Debug, Component, Default, Reflect)]
pub struct Collider {
    #[reflect(ignore)]
    pub meshes: Vec<Box<dyn SuperMesh>>,
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
>>>>>>> Stashed changes
    }
}
