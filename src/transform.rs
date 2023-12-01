use bevy_ecs::prelude::*;

use crate::{space::{Position, Velocity, Rotation}, Update};


#[derive(Default)]
pub struct TransformSettings {
    pub use_gravity: bool, 
    pub auto_update: bool,
}



#[derive(Bundle, Default)]
pub struct Transform {
    pub position: Position,
    pub velocity: Velocity,
    pub rotation: Rotation,
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
