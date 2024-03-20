use bevy_ecs::{component::Component, reflect::ReflectComponent};

use bevy_reflect::Reflect;
use engine::space;
use serde::Serialize;

/// A group of settings for controlling gravitational force for an entity.
///
#[derive(Debug, Component, Clone, Copy, Reflect, Serialize)]
#[reflect(Component)]
pub struct GravitySettings {
    pub force: space::Vector2,
    pub force_cap: f32,
}

impl Default for GravitySettings {
    fn default() -> Self {
        Self {
            force: Default::default(),
            force_cap: Default::default(),
        }
    }
}
