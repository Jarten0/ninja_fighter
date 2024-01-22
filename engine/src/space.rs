use core::fmt;
use std::{
    ops::{Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    time::Duration,
};

use bevy_ecs::component::Component;
use ggez::graphics::Vertex as DrawVertex;
use nalgebra::base;

pub use pos::Position;
pub use rtt::Rotation;
pub use scl::Scale;
pub use vel::Velocity;

#[derive(Debug, Clone, Copy)]
pub struct Vector2(mint::Vector2<f32>);

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            0: mint::Vector2 { x, y },
        }
    }

    pub fn translate(&mut self, translation: &Vector2) {
        self.x += translation.x;
        self.y += translation.y;
    }

    /// Linear
    pub fn lerp(&mut self, translation: &Vector2, time: Duration) {
        // <Vector2 as Deref>::Target
    }
}

impl Vector2 {
    pub fn up() -> Self {
        Vector2 {
            0: mint::Vector2 { x: 0.0, y: -1.0 },
        }
    }
    pub fn down() -> Self {
        Vector2 {
            0: mint::Vector2 { x: 0.0, y: 1.0 },
        }
    }
    pub fn left() -> Self {
        Vector2 {
            0: mint::Vector2 { x: -1.0, y: 0.0 },
        }
    }
    pub fn right() -> Self {
        Vector2 {
            0: mint::Vector2 { x: 1.0, y: 0.0 },
        }
    }
    pub fn zero() -> Self {
        Vector2 {
            0: mint::Vector2 { x: 0.0, y: 0.0 },
        }
    }
    pub fn one() -> Self {
        Vector2 {
            0: mint::Vector2 { x: 1.0, y: 1.0 },
        }
    }
}

impl Default for Vector2 {
    fn default() -> Self {
        Self::new(0.0, 0.0)
    }
}

impl Deref for Vector2 {
    type Target = mint::Vector2<f32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Vector2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Into<(f32, f32)> for Vector2 {
    /// Returns the [`Vector2`] as a tuple struct, where 0 = vec.x and 1 = vec.y
    fn into(self) -> (f32, f32) {
        (self.x, self.y)
    }
}

impl From<(f32, f32)> for Vector2 {
    fn from((x, y): (f32, f32)) -> Self {
        Vector2::new(x, y)
    }
}

impl fmt::Display for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0.x, self.0.y)
    }
}

impl Add for Vector2 {
    type Output = Vector2;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.x += rhs.x;
        self.y += rhs.y;
        self
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vector2 {
    type Output = Vector2;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self
    }
}

impl SubAssign for Vector2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
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

impl From<Vector2> for Vertex {
    fn from(value: Vector2) -> Self {
        Self { 0: value }
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

mod pos;

mod vel;

mod rtt;

mod scl;
