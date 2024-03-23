mod box_collider;
mod convex_mesh;
mod gravity_settings;

pub use box_collider::BoxCollider;
pub use convex_mesh::ConvexMesh;
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
    }
}
