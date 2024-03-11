use bevy_ecs::component::Component;

use bevy_reflect::Reflect;
use engine::space;

/// A group of settings for controlling gravitational force for an entity.
///
#[derive(Debug, Component, Clone, Copy, Reflect)]
pub struct GravitySettings {
    pub force: space::Vector2,
    pub force_cap: f32,
}

impl Default for GravitySettings {
    fn default() -> Self {
        Self {
            force: space::Vector2::zero(),
            force_cap: 10.0,
        }
    }
}
