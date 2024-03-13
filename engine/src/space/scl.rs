use super::Vector2;
use bevy_ecs::component::Component;
use bevy_ecs::reflect::ReflectComponent;
use bevy_reflect::Reflect;
use serde::Deserialize;
use serde::Serialize;
use std::ops::Deref;
use std::ops::DerefMut;

#[derive(Debug, Component, Clone, Copy, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
pub struct Scale(pub(crate) Vector2);

impl Default for Scale {
    fn default() -> Self {
        Self(Vector2::new(1.0, 1.0))
    }
}

impl Deref for Scale {
    type Target = Vector2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Scale {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Scale {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            0: Vector2::new(x, y),
        }
    }
}
