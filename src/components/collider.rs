use bevy_ecs::{bundle::Bundle, component::Component};


#[derive(Default, Clone, Copy, Bundle)]
pub struct Collider {
    pub gravity: GravitySettings,
}

/// A group of settings for controlling gravitational force for an entity.
/// 
#[derive(Component, Clone, Copy)]
pub struct GravitySettings {
    pub force: crate::space::Vector2,
}

impl Default for GravitySettings {
    fn default() -> Self {
        Self { 
            force: crate::space::Vector2::zero()
        }
    }
}

