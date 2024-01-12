use bevy_ecs::prelude::*;

use engine::space::{Position, Rotation, Scale, Velocity};

#[derive(Debug, Component, Default, Clone, Copy)]
pub struct TransformSettings {
    pub auto_update: bool,
}

#[derive(Bundle, Default, Clone, Copy, Debug)]
pub struct Transform {
    pub position: Position,
    pub velocity: Velocity,
    pub rotation: Rotation,
    pub scale: Scale,
    pub settings: TransformSettings,
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
