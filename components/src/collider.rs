pub mod collider_mesh;
pub mod gravity_settings;

use engine::space::{Vector2, Velocity};

use engine::GgezInterface;

use bevy_ecs::{
    bundle::Bundle,
    system::{Query, Res},
};

use self::collider_mesh::ColliderMesh;
use self::gravity_settings::GravitySettings;

use engine::space::Transform;

#[derive(Debug, Clone, Bundle)]
pub struct Collider {
    gravity: gravity_settings::GravitySettings,
    mesh: collider_mesh::ColliderMesh,
}

impl Collider {
    pub fn new(engine: &GgezInterface) -> Self {
        let transform = Transform::default();

        let gravity = GravitySettings {
            force: Vector2::down(),
            force_cap: 5.0,
        };

        Self {
            gravity,
            mesh: ColliderMesh::new(&engine.get_context().gfx),
        }
    }
}

pub fn update(mut query: Query<(&mut Velocity, &GravitySettings)>, engine: Res<GgezInterface>) {
    for (mut velocity, gravity_settings) in query.iter_mut() {
        // velocity.translate(&gravity_settings.force * engine.get_context().time.delta().as_secs())
    }
}
