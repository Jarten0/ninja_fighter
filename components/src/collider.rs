pub mod collider_mesh;
pub mod gravity_settings;

use std::any::Any;

use bevy_reflect::Reflect;
use engine::space::{self, Vector2, Velocity};

use engine::GgezInterface;

use bevy_ecs::{
    bundle::Bundle,
    system::{Query, Res},
};

use self::collider_mesh::ConvexColliderMesh;
use self::gravity_settings::GravitySettings;

use engine::space::Transform;

pub trait Collider {
    fn drawable(&self) -> Option<&dyn Any>;
}

#[derive(Debug, Clone, Bundle, Reflect, Default)]
pub struct ColliderBundle {
    gravity: gravity_settings::GravitySettings,
    mesh: collider_mesh::ConvexColliderMesh,
}

impl ColliderBundle {
    pub fn new(engine: &GgezInterface, vertices: &[ggez::graphics::Vertex]) -> Self {
        let transform = Transform::default();

        let gravity = GravitySettings {
            force: space::DOWN,
            force_cap: 5.0,
        };

        let indices = Vec::new();

        Self {
            gravity,
            mesh: ConvexColliderMesh::new_with_drawable(
                &engine.get_context().gfx,
                vertices,
                &indices,
            ),
        }
    }
}

pub fn update(mut query: Query<(&mut Velocity, &GravitySettings)>, engine: Res<GgezInterface>) {
    // for (mut velocity, gravity_settings) in query.iter_mut() {
    // velocity.translate(&gravity_settings.force * engine.get_context().time.delta().as_secs())
    // }
}
