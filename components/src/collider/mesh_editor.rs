use crate::collider::*;
use bevy_ecs::entity::Entity;
use bevy_ecs::system::Resource;
use bevy_reflect::Reflect;
use engine::scene::ObjectID;
use std::collections::HashMap;

#[derive(Debug, Clone, Resource, Reflect, Default)]
pub struct MeshEditor {
    pub focus: FocusState,
    /// Key = mesh, Value = vertex index in mesh vertices
    vertices_to_draw: HashMap<Entity, (ObjectID, usize)>,
}

#[derive(Debug, Clone, Reflect, Default)]
pub enum FocusState {
    #[default]
    Idle,
    FocusedOnEntity {
        focused_entity: Entity,
    },
    FocusedOnMesh {
        focused_entity: Entity,
        focused_mesh_id: ObjectID,
        focused_vertex_index: Option<usize>,
    },
}
