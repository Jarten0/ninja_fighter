use bevy_ecs::{component::Component, reflect::ReflectComponent};
use bevy_reflect::Reflect;
use engine::space::Vector2;

use crate::collider::collider_mesh::ColliderMesh;

#[derive(Debug, Default, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct BoxCollider {
    mesh: ColliderMesh,
}

impl BoxCollider {
    pub fn new(scale: Vector2) -> Self {
        let mut mesh = ColliderMesh::default();
        // mesh.add_vertex(Vertex::from(Vector2::new(0.0, 0.0)));
        // mesh.add_vertex(Vertex::from(Vector2::new(1.0, 0.0) * scale.x));
        // mesh.add_vertex(Vertex::from(Vector2::new(1.0 * scale.x, 1.0 * scale.y)));
        // mesh.add_vertex(Vertex::from(Vector2::new(0.0, 1.0) * scale.y));
        Self { mesh }
    }
}
