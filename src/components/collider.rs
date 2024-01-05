pub mod collider_mesh;
pub mod gravity_settings;
pub mod meta_vertex;

use crate::engine::space::Vector2;

use crate::engine::Engine;

use bevy_ecs::bundle::Bundle;

use self::collider_mesh::ColliderMesh;
use self::gravity_settings::GravitySettings;

#[derive(Debug, Clone, Bundle)]
pub struct Collider {
    gravity: gravity_settings::GravitySettings,
    mesh: collider_mesh::ColliderMesh,
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
