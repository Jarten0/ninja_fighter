use bevy_ecs::prelude::*;

#[derive(Default, Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {}

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