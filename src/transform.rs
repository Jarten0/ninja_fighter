use bevy_ecs::prelude::*;

use crate::Update;

#[derive(Default, Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Update<(&mut Position, &Velocity)> for Position {
    fn update(mut query: Query<(&mut Position, &Velocity)>) {}
}

#[derive(Component, Default)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Default)]
pub struct Rotation {
    pub angle: f32,
}

#[derive(Bundle, Default)]
pub struct Transform {
    pub position: Position,
    pub velocity: Velocity,
    pub rotation: Rotation,
}
