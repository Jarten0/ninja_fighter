pub mod collider_mesh;
pub mod gravity_settings;

use std::any::Any;

use bevy_reflect::Reflect;
use engine::space::{Vector2, Velocity};

use engine::GgezInterface;

use bevy_ecs::{
    bundle::Bundle,
    system::{Query, Res},
};

use self::collider_mesh::ColliderMesh;
use self::gravity_settings::GravitySettings;

use engine::space::Transform;

pub trait Collider {
    fn drawable(&self) -> Option<&dyn Any>;
}

#[derive(Debug, Clone, Bundle, Reflect, Default)]
pub struct ColliderBundle {
    gravity: gravity_settings::GravitySettings,
    mesh: collider_mesh::ColliderMesh,
}

impl ColliderBundle {
    pub fn new(
        engine: &GgezInterface,
        vertices: &[ggez::graphics::Vertex],
        indices: &[u32],
    ) -> Self {
        let transform = Transform::default();

        let gravity = GravitySettings {
            force: Vector2::down(),
            force_cap: 5.0,
        };

        Self {
            gravity,
            mesh: ColliderMesh::new(&engine.get_context().gfx, vertices, indices),
        }
    }
}

pub fn update(mut query: Query<(&mut Velocity, &GravitySettings)>, engine: Res<GgezInterface>) {
    for (mut velocity, gravity_settings) in query.iter_mut() {
        // velocity.translate(&gravity_settings.force * engine.get_context().time.delta().as_secs())
    }
}
