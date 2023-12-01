use bevy_ecs::component::Component;
use ggez::mint::Vector2;

#[derive(Default, Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
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

pub struct Vector(Vector2<>);