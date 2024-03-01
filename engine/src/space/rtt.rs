use bevy_ecs::component::Component;
use bevy_reflect::Reflect;
use serde::Deserialize;
use serde::Serialize;
use std::ops::Deref;
use std::ops::DerefMut;

#[derive(Debug, Component, Default, Clone, Copy, Serialize, Deserialize, Reflect)]
pub struct Rotation(f32);

impl Rotation {
    pub fn new(angle: f32) -> Self {
        Self { 0: angle }
    }
}

impl Deref for Rotation {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Rotation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
