use super::Vector2;
use bevy_ecs::component::Component;
use std::ops::Deref;
use std::ops::DerefMut;

#[derive(Debug, Default, Component, Clone, Copy)]
pub struct Position(Vector2);

impl Deref for Position {
    type Target = Vector2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Position {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            0: Vector2::new(x, y),
        }
    }
}
