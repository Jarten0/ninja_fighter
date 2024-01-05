use std::ops::{Deref, DerefMut};

use bevy_ecs::component::Component;
use ggez::graphics::Vertex as DrawVertex;
use mint::Vector2 as mVec;

#[derive(Debug, Clone, Copy)]
pub struct Vector2(mVec<f32>);

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { 0: mVec { x, y } }
    }

    pub fn translate(&mut self, translation: &Vector2) {
        self.x += translation.x;
        self.y += translation.y;
    }
}

impl Vector2 {
    pub fn up() -> Self {
        Vector2 {
            0: mVec { x: 0.0, y: -1.0 },
        }
    }
    pub fn down() -> Self {
        Vector2 {
            0: mVec { x: 0.0, y: 1.0 },
        }
    }
    pub fn left() -> Self {
        Vector2 {
            0: mVec { x: -1.0, y: 0.0 },
        }
    }
    pub fn right() -> Self {
        Vector2 {
            0: mVec { x: 1.0, y: 0.0 },
        }
    }
    pub fn zero() -> Self {
        Vector2 {
            0: mVec { x: 0.0, y: 0.0 },
        }
    }
    pub fn one() -> Self {
        Vector2 {
            0: mVec { x: 1.0, y: 1.0 },
        }
    }
}

impl Default for Vector2 {
    fn default() -> Self {
        Self::new(0.0, 0.0)
    }
}

impl Deref for Vector2 {
    type Target = mVec<f32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Vector2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Vertex(Vector2);

impl Deref for Vertex {
    type Target = Vector2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Vertex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Into<DrawVertex> for Vertex {
    fn into(self) -> DrawVertex {
        DrawVertex {
            position: [self.x, self.y],
            uv: [10.0, 10.0],
            color: [0.0, 0.0, 0.0, 1.0],
        }
    }
}

#[derive(Debug, Default, Component, Clone, Copy)]
pub struct Position(Vector2);

impl Deref for Position {
    type Target = Vector2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Position {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            0: Vector2::new(x, y),
        }
    }
}

#[derive(Debug, Component, Default, Clone, Copy)]
pub struct Velocity(Vector2);

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            0: Vector2::new(x, y),
        }
    }
}

impl Deref for Velocity {
    type Target = Vector2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Velocity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Component, Default, Clone, Copy)]
pub struct Rotation(f32);

impl Rotation {
    pub fn new(angle: f32) -> Self {
        Self { 0: angle }
    }
}

impl Deref for Rotation {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Rotation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Component, Clone, Copy)]
pub struct Scale(Vector2);

impl Default for Scale {
    fn default() -> Self {
        Self(Vector2::new(1.0, 1.0))
    }
}

impl Deref for Scale {
    type Target = Vector2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Scale {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Scale {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            0: Vector2::new(x, y),
        }
    }
}
