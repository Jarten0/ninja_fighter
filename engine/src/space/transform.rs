use bevy_ecs::prelude::*;
use bevy_reflect::Reflect;

use super::{Position, Rotation, Scale, Vector2, Velocity};

#[derive(Debug, Component, Default, Clone, Copy, Reflect)]
pub struct TransformSettings {
    pub auto_update: bool,
}

impl TransformSettings {
    pub const fn new() -> Self {
        Self { auto_update: false }
    }
}

#[derive(Bundle, Default, Clone, Copy, Debug)]
pub struct Transform {
    pub position: Position,
    pub velocity: Velocity,
    pub rotation: Rotation,
    pub scale: Scale,
    pub settings: TransformSettings,
}

pub static DEFAULT_TRANSFORM: Transform = Transform::new();

impl Transform {
    pub const fn new() -> Self {
        Transform {
            position: Position(Vector2::new(0.0, 0.0)),
            velocity: Velocity(Vector2::new(0.0, 0.0)),
            rotation: Rotation(0.0),
            scale: Scale(Vector2::new(1.0, 1.0)),
            settings: TransformSettings::new(),
        }
    }
}

pub fn update(mut query: Query<(&mut Position, &Velocity, &TransformSettings)>) {
    for (mut position, velocity, transform_settings) in query.iter_mut() {
        if transform_settings.auto_update {
            position.translate(velocity);
        }
    }
}

impl Into<ggez::graphics::Transform> for Transform {
    fn into(self) -> ggez::graphics::Transform {
        ggez::graphics::Transform::Values {
            dest: mint::Point2::<f32> {
                x: self.position.x,
                y: self.position.y,
            },
            rotation: *self.rotation,
            scale: mint::Vector2::<f32> {
                x: self.scale.x,
                y: self.scale.y,
            },
            offset: mint::Point2 { x: 0.0, y: 0.0 },
        }
    }
}
