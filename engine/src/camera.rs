use std::ops::Deref;

use crate::space::{self, Vector2};
use bevy_ecs::system::Resource;

#[allow(unused)]
#[derive(Resource)]
pub struct Camera {
    pub position: space::Position,
    pub velocity: space::Velocity,
    pub zoom: f32, // %100 or 1.0 zoom = 1cm/px or 1m/100px
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Default::default(),
            velocity: Default::default(),
            zoom: 1.0,
        }
    }
}

impl Camera {
    /// Returns the exact distance an object must be moved to keep up with the camera
    pub fn get_offset(&self) -> Vector2 {
        -*self.position.deref()
    }
}
