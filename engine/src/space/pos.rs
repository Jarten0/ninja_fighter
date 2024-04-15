use super::Vector2;
use bevy_ecs::component::Component;
use bevy_ecs::reflect::ReflectComponent;
use bevy_reflect::FromReflect;
use bevy_reflect::Reflect;
use serde::Deserialize;
use serde::Serialize;
use std::ops::Deref;
use std::ops::DerefMut;

#[derive(Debug, Default, Component, Clone, Copy, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
pub struct Position(pub(crate) Vector2);

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

impl From<Vector2> for Position {
    fn from(value: Vector2) -> Self {
        Position(value)
    }
}
