use super::Vector2;
use bevy_ecs::component::Component;
use serde::Deserialize;
use serde::Serialize;
use std::ops::Deref;
use std::ops::DerefMut;

#[derive(Debug, Component, Default, Clone, Copy, Serialize, Deserialize)]
pub struct Velocity(Vector2);

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            0: Vector2::new(x, y),
        }
    }
}

impl Deref for Velocity {
    type Target = Vector2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Velocity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
