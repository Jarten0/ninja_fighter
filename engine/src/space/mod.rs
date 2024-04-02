//TODO: Fix these docs lol
//! main docs, but i aint finishing it rn
//! transform is split into several components, position, velocity, scale and rotation
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

use bevy_ecs::prelude::*;
use log::error;
use mint::Point2;
pub use pos::Position;
pub use rtt::Rotation;
pub use scl::Scale;
pub use transform::{Transform, TransformSettings, DEFAULT_TRANSFORM};
pub use vel::Velocity;
pub use vtx::Vertex;

use bevy_reflect::{
    DynamicStruct, DynamicTypePath, FieldIter, FromReflect, GetTypeRegistration, NamedField,
    Reflect, ReflectRef, StructInfo, TypeInfo, TypePath, Typed,
};
use core::fmt;
use once_cell::sync::Lazy;
use serde::{de::Visitor, ser::SerializeStruct, Deserialize, Serialize};
use std::any::Any;
use std::ops::{Add, AddAssign, Deref, DerefMut, Div, Mul, Neg, Sub, SubAssign};
use std::time::Duration;

// Struct block //

#[derive(Debug, Clone, Copy, TypePath, Component, PartialEq, PartialOrd)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

/// An angle, working in tandem with Vector2's and [`ggez`].
///
/// This is used to demystify any ambiguity by giving a direct way of converting from types of rotations easily.
#[derive(Debug, Default, Clone, Copy, Reflect, PartialEq, PartialOrd)]
pub struct Angle {
    degrees: f32,
}

impl Angle {
    /// Returns the angle as a degree between 0 and 360, with 0 pointing directly up and each degree moving clockwise
    pub fn degrees_north(&self) -> f32 {
        self.degrees + 90.0
    }

    /// Returns the angle as a degree between -360 and +360, with 0/360/-360 pointing east.
    /// This is the default that is used between math libraries, and the default for the Angle struct.
    pub fn degrees(&self) -> f32 {
        self.degrees
    }

    /// Sets the angle to a value between 0 and 360
    pub fn clamp_360(&self) -> Self {
        Self::from((self.degrees / 360.0).fract() * 360.0)
    }

    /// Sets the angle to a value between -180 and 180, using
    pub fn clamp_180(&self) -> Self {
        let clamp_360 = self.clamp_360();
        Angle::from(if clamp_360 > 180.into() {
            *clamp_360 - 360.0
        } else {
            *clamp_360
        })
    }

    /// Returns the degrees with 0 pointing up (or north-origin angle), and incrementing clockwise.
    ///
    /// This might be the more intuitive solution you're looking for, but if you want to create an east-origin angle, use [`from()`](Angle::from())
    pub fn from_degrees_north(value: f32) -> Self {
        Self {
            degrees: value - 90.0,
        }
    }
}

impl From<f32> for Angle {
    /// Converts a float value to an angle, as the default east-origin angle.
    ///
    /// Do not feed a north-origin value, use [`from_degrees_north()`](Angle::from_degrees_north) instead
    fn from(value: f32) -> Self {
        Self { degrees: value }
    }
}
impl From<f64> for Angle {
    /// Converts a float value to an angle, as the default east-origin angle.
    ///
    /// Do not feed a north-origin value, use [`from_degrees_north()`](Angle::from_degrees_north) instead
    fn from(value: f64) -> Self {
        Self {
            degrees: value as f32,
        }
    }
}
impl From<i32> for Angle {
    /// Converts a float value to an angle, as the default east-origin angle.
    ///
    /// Do not feed a north-origin value, use [`from_degrees_north()`](Angle::from_degrees_north) instead
    fn from(value: i32) -> Self {
        Self {
            degrees: value as f32,
        }
    }
}
impl Deref for Angle {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.degrees
    }
}
impl DerefMut for Angle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.degrees
    }
}

pub static UP: Vector2 = Vector2 { x: 0.0, y: 1.0 };
pub static DOWN: Vector2 = Vector2 { x: 0.0, y: -1.0 };
pub static LEFT: Vector2 = Vector2 { x: -1.0, y: 0.0 };
pub static RIGHT: Vector2 = Vector2 { x: 1.0, y: 0.0 };
pub static ZERO: Vector2 = Vector2 { x: 0.0, y: 0.0 };
pub static ONE: Vector2 = Vector2 { x: 1.0, y: 1.0 };

impl Vector2 {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Translates the given vertex a given amount, and returns a reference to that initial vector
    pub fn translate(&mut self, translation: &Vector2) -> &mut Self {
        self.x += translation.x;
        self.y += translation.y;
        self
    }
    /// Translates the given vertex a given amount, and returns a reference to that initial vector
    pub fn translated(&self, translation: &Vector2) -> Self {
        Vector2 {
            x: self.x + translation.x,
            y: self.y + translation.y,
        }
    }

    pub fn set(&mut self, set: Vector2) -> &mut Self {
        self.x = set.x;
        self.y = set.y;
        self
    }

    pub fn scale(&mut self, scale: &Vector2) -> &mut Self {
        self.x *= scale.x;
        self.y *= scale.y;
        self
    }

    pub fn scaled(&self, scale: &Vector2) -> Self {
        Vector2 {
            x: self.x * scale.x,
            y: self.y * scale.y,
        }
    }

    pub fn magnitude(&self) -> f32 {
        f32::hypot(self.x, self.y)
    }

    pub fn normalized(&self) -> Vector2 {
        let magnitude = self.magnitude();
        Self {
            x: self.x / magnitude,
            y: self.y / magnitude,
        }
    }

    /// Linear
    pub fn lerp(&mut self, _translation: &Vector2, _time: Duration) {
        // <Vector2 as Deref>::Target
        todo!()
    }

    pub fn angle(&self) -> f32 {
        libm::atan2(self.y as f64, self.x as f64).to_degrees() as f32
    }

    /// Gets the degree angle between two vectors.
    ///
    /// Returns a positive number between 0 and 360, if
    pub fn get_angle_between(self, other: Self) -> Angle {
        Angle::from(self.angle() - other.angle())
    }

    /// Creates a new [`Vector2`] with the position of `self` as a new origin and `other` as the value
    pub fn inverse_sum(self, other: Self) -> Vector2 {
        other - self
    }

    pub fn dot(self, other: Self) -> f32 {
        f32::cos(Vector2::get_angle_between(self, other).degrees_north()) // cos0
            * self.magnitude()
            * other.magnitude()
    }

    pub fn cross_product(self, other: Self) -> Self {
        ONE * (f32::sin(self.get_angle_between(other).abs()) * self.magnitude() * other.magnitude())
    }
}

// Initialization block //

impl Default for Vector2 {
    fn default() -> Self {
        ZERO
    }
}

impl Into<mint::Vector2<f32>> for Vector2 {
    fn into(self) -> mint::Vector2<f32> {
        mint::Vector2 {
            x: self.x,
            y: self.y,
        }
    }
}
impl Into<[f32; 2]> for Vector2 {
    fn into(self) -> [f32; 2] {
        [self.x, self.y]
    }
}

impl From<[f32; 2]> for Vector2 {
    fn from(value: [f32; 2]) -> Self {
        Self {
            x: value[0],
            y: value[1],
        }
    }
}

impl From<mint::Vector2<f32>> for Vector2 {
    fn from(value: mint::Vector2<f32>) -> Self {
        Self {
            x: value.x,
            y: value.x,
        }
    }
}

impl From<(f32, f32)> for Vector2 {
    fn from((x, y): (f32, f32)) -> Self {
        Vector2::new(x, y)
    }
}

impl From<Point2<f32>> for Vector2 {
    fn from(value: Point2<f32>) -> Self {
        Vector2::new(value.x, value.y)
    }
}

impl From<&Vector2> for Box<Vector2> {
    fn from(value: &Vector2) -> Self {
        Box::new(value.to_owned())
    }
}

impl fmt::Display for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
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
impl Add<&Self> for Vector2 {
    type Output = Vector2;

    fn add(mut self, rhs: &Self) -> Self::Output {
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
impl AddAssign<&Self> for Vector2 {
    fn add_assign(&mut self, rhs: &Self) {
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
impl Sub<&Self> for Vector2 {
    type Output = Vector2;

    fn sub(mut self, rhs: &Self) -> Self::Output {
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
impl SubAssign<&Self> for Vector2 {
    fn sub_assign(&mut self, rhs: &Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<f32> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl Div<f32> for Vector2 {
    type Output = Vector2;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Neg for Vector2 {
    type Output = Vector2;

    fn neg(mut self) -> Self::Output {
        self.x = -self.x;
        self.y = -self.y;
        self
    }
}

// Serialize block //

impl Serialize for Vector2 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut vec = serializer.serialize_struct("Vector2", 2)?;
        let _ = vec.serialize_field("x", &self.x);
        let _ = vec.serialize_field("y", &self.y);
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

// Reflection block //

static TYPE_INFO: Lazy<TypeInfo> =
    Lazy::new(|| bevy_reflect::TypeInfo::Struct(TUPLE_STRUCT_INFO.clone()));

static STRUCT_INFO: Lazy<bevy_reflect::TypeInfo> =
    Lazy::new(|| bevy_reflect::TypeInfo::Struct(TUPLE_STRUCT_INFO.clone()));

static TUPLE_STRUCT_INFO: Lazy<StructInfo> = Lazy::new(|| {
    StructInfo::new::<Vector2>(&[NamedField::new::<f32>("x"), NamedField::new::<f32>("y")])
});

impl bevy_reflect::Struct for Vector2 {
    fn field_len(&self) -> usize {
        2
    }

    fn iter_fields(&self) -> FieldIter<'_> {
        bevy_reflect::FieldIter::new(self)
    }

    fn clone_dynamic(&self) -> DynamicStruct {
        let mut dynamic_struct = DynamicStruct::default();
        dynamic_struct.set_represented_type(Some(self.get_represented_type_info().unwrap()));
        dynamic_struct.insert("x", self.x);
        dynamic_struct.insert("y", self.y);
        dynamic_struct
    }

    fn field_at(&self, index: usize) -> Option<&dyn Reflect> {
        match index {
            0 => Some(self.x.as_reflect()),
            1 => Some(self.y.as_reflect()),
            _ => None,
        }
    }

    fn field_at_mut(&mut self, index: usize) -> Option<&mut dyn Reflect> {
        match index {
            0 => Some(self.x.as_reflect_mut()),
            1 => Some(self.y.as_reflect_mut()),
            _ => None,
        }
    }

    fn name_at(&self, index: usize) -> Option<&str> {
        match index {
            0 => Some("x"),
            1 => Some("y"),
            _ => None,
        }
    }

    fn field(&self, name: &str) -> Option<&dyn Reflect> {
        match name {
            "x" => Some(self.x.as_reflect()),
            "y" => Some(self.y.as_reflect()),
            _ => None,
        }
    }

    fn field_mut(&mut self, name: &str) -> Option<&mut dyn Reflect> {
        match name {
            "x" => Some(self.x.as_reflect_mut()),
            "y" => Some(self.y.as_reflect_mut()),
            _ => None,
        }
    }
}

impl Reflect for Vector2 {
    fn get_represented_type_info(&self) -> Option<&'static bevy_reflect::TypeInfo> {
        Some(&STRUCT_INFO)
    }

    fn into_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self as Box<dyn Any>
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn Any
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn Any
    }

    fn into_reflect(self: Box<Self>) -> Box<dyn Reflect> {
        self as Box<dyn Reflect>
    }

    fn as_reflect(&self) -> &dyn Reflect {
        self as &dyn Reflect
    }

    fn as_reflect_mut(&mut self) -> &mut dyn Reflect {
        self as &mut dyn Reflect
    }

    fn apply(&mut self, value: &dyn Reflect) {
        let downcast_ref = match value.downcast_ref::<Self>() {
            Some(some) => some,
            None => {
                error!(
                    "Could not apply Reflect to Vector2: value is not a Vector2. value type: {}",
                    value.reflect_type_path()
                );
                return;
            }
        };

        self.x = downcast_ref.x;
        self.y = downcast_ref.y;
    }

    fn set(&mut self, value: Box<dyn Reflect>) -> Result<(), Box<dyn Reflect>> {
        if !value.is::<Self>() {
            return Err(value);
        }

        self.apply(&*value);
        Ok(())
    }

    fn reflect_ref(&self) -> bevy_reflect::ReflectRef {
        ReflectRef::Struct(self)
    }

    fn reflect_mut(&mut self) -> bevy_reflect::ReflectMut {
        bevy_reflect::ReflectMut::Struct(self)
    }

    fn reflect_owned(self: Box<Self>) -> bevy_reflect::ReflectOwned {
        bevy_reflect::ReflectOwned::Struct(self)
    }

    fn clone_value(&self) -> Box<dyn Reflect> {
        <Vector2 as Reflect>::into_reflect(Into::<Box<Vector2>>::into(self))
    }

    fn reflect_partial_eq(&self, value: &dyn Reflect) -> Option<bool> {
        Some(self.eq(value.downcast_ref()?)) // TODO: make sure that downcast_ref works with traits
    }

    fn serializable(&self) -> Option<bevy_reflect::serde::Serializable> {
        Some(bevy_reflect::serde::Serializable::Borrowed(self))
    }
}

impl FromReflect for Vector2 {
    fn from_reflect(reflect: &dyn Reflect) -> Option<Self> {
        if let ReflectRef::Struct(dyn_struct) = reflect.reflect_ref() {
            Some(Self {
                x: (f32::from_reflect(dyn_struct.field("x")?))?,
                y: (f32::from_reflect(dyn_struct.field("y")?))?,
            })
        } else {
            None
        }
    }
}

impl GetTypeRegistration for Vector2 {
    fn get_type_registration() -> bevy_reflect::TypeRegistration {
        bevy_reflect::TypeRegistration::of::<Self>()
    }
}

impl Typed for Vector2 {
    fn type_info() -> &'static bevy_reflect::TypeInfo {
        &TYPE_INFO
    }
}
