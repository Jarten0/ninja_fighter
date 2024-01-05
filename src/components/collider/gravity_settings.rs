use bevy_ecs::component::Component;

use crate::engine::space;

/// A group of settings for controlling gravitational force for an entity.
///
#[derive(Debug, Component, Clone, Copy)]
pub struct GravitySettings {
    pub force: space::Vector2,
}

impl Default for GravitySettings {
    fn default() -> Self {
        Self {
            force: space::Vector2::zero(),
        }
    }
}
