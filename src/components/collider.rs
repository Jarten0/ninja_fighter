pub mod collider_mesh;
pub mod gravity_settings;
pub mod meta_vertex;

use crate::engine::space::{Position, Vector2, Velocity};

use crate::engine::Engine;

use bevy_ecs::bundle::Bundle;
use bevy_ecs::query::With;
use bevy_ecs::system::Query;

use self::collider_mesh::ColliderMesh;
use self::gravity_settings::GravitySettings;

use super::transform::Transform;

#[derive(Debug, Clone, Bundle)]
pub struct Collider {
    gravity: GravitySettings,
    mesh: ColliderMesh,
}

impl Collider {
    pub fn new(engine: &Engine) -> Self {
        Self {
            gravity: GravitySettings {
                force: Vector2::default(),
            },
            mesh: ColliderMesh::new(&engine.get_context().gfx),
        }
    }
}

fn update(
    mut query: Query<(
        &mut ColliderMesh,
        &GravitySettings,
        &Position,
        &mut Velocity,
    )>,
) {
}
