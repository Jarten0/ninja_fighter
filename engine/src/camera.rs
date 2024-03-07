use crate::space;
use bevy_ecs::component::Component;

#[allow(unused)]
#[derive(Component)]
pub struct Camera {
    position: space::Position,
    velocity: space::Velocity,
    zoom: f32,
}
