use bevy_ecs::prelude::*;

use crate::{space::{Position, Velocity, Rotation, Scale}, Update};


#[derive(Default)]
pub struct TransformSettings {
    pub use_gravity: bool, 
    pub auto_update: bool,
}



#[derive(Bundle, Default, Clone, Copy)]
pub struct Transform {
    pub position: Position,
    pub velocity: Velocity,
    pub rotation: Rotation,
    pub scale: Scale,
}

impl Update<(&mut Position, &Velocity)> for Transform {
    // fn translate() {
    //     position.x += veloc

    // }

    fn update(mut query: Query<(&mut Position, &Velocity)>) {
        for (mut position, velocity) in query.iter_mut() {
        }

    }
}

impl Into<ggez::graphics::Transform> for Transform {
    fn into(self) -> ggez::graphics::Transform {
        ggez::graphics::Transform::Values { 
            dest: mint::Point2::<f32> { x: self.position.x, y: self.position.y }, 
            rotation: *self.rotation, 
            scale: mint::Vector2::<f32> { x: self.scale.x, y: self.scale.y }, 
            offset: mint::Point2 { x: 0.0, y: 0.0 } }
    }
}