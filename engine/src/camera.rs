use bevy_ecs::component::Component;

use crate::space;

#[derive(Component)]
pub struct Camera {
    position: space::Position,
    velocity: space::Velocity,
    zoom: f32,
}
