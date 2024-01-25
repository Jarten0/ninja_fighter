//! main docs, but i aint finishing it rn
//! transform is split into three components, position, velocity, scale and rotation
//! this is where those components are hosted so they're pretty much built into the engine
//! reason being the [`Vector2`] struct is here, and the rest pretty much rely entirely on it (aside from rotation)
//! anyways this module contains [`Vector2`]

// don't you love it when you have 17 impl blocks in one file yay fun
// anyways i split them up into several blocks so as to make organization a little bit easier yw

pub(crate) mod pos;
pub(crate) mod rtt;
pub(crate) mod scl;
pub(crate) mod transform;
pub(crate) mod vel;
pub(crate) mod vtx;

pub use pos::Position;
pub use rtt::Rotation;
pub use scl::Scale;
pub use transform::{Transform, TransformSettings};
pub use vel::Velocity;
pub use vtx::Vertex;

use bevy_ecs::component::Component;
use core::fmt;
use ggez::graphics::Vertex as DrawVertex;
use nalgebra::base;
use serde::{de::Visitor, ser::SerializeStruct, Deserialize, Serialize};
use std::ops::{
    Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign,
};
use std::time::Duration;

// Struct block //

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

// Deref block //

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

// Initialization block //

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

// Operator block //

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

// Serialize block //

impl Serialize for Vector2 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut vec = serializer.serialize_struct("Vector2", 2)?;
        vec.serialize_field("x", &self.x);
        vec.serialize_field("y", &self.y);
        vec.end()
    }
}

impl<'de> Deserialize<'de> for Vector2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(deserializer.deserialize_struct("Vector2", &["x", "y"], F32Visitor {})?)
    }
}
struct F32Visitor;

impl<'de> Visitor<'de> for F32Visitor {
    type Value = Vector2;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("any f32 value i really dont want to write deserialization code ughhh")
    }
}
