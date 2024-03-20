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
pub use pos::Position;
pub use rtt::Rotation;
pub use scl::Scale;
pub use transform::{update, Transform, TransformSettings, DEFAULT_TRANSFORM};
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
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};
use std::time::Duration;

// Struct block //

#[derive(Debug, Clone, Copy, TypePath, Component, PartialEq, PartialOrd)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

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

    /// Linear
    pub fn lerp(&mut self, _translation: &Vector2, _time: Duration) {
        // <Vector2 as Deref>::Target
        todo!()
    }
}

// Initialization block //

impl Vector2 {
    pub fn up() -> Self {
        Vector2 { x: 0.0, y: -1.0 }
    }
    pub fn down() -> Self {
        Vector2 { x: 0.0, y: 1.0 }
    }
    pub fn left() -> Self {
        Vector2 { x: -1.0, y: 0.0 }
    }
    pub fn right() -> Self {
        Vector2 { x: 1.0, y: 0.0 }
    }
    pub fn zero() -> Self {
        Vector2 { x: 0.0, y: 0.0 }
    }
    pub fn one() -> Self {
        Vector2 { x: 1.0, y: 1.0 }
    }
}

impl Default for Vector2 {
    fn default() -> Self {
        Self::new(0.0, 0.0)
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

impl From<mint::Vector2<f32>> for Vector2 {
    fn from(value: mint::Vector2<f32>) -> Self {
        Self {
            x: value.x,
            y: value.x,
        }
    }
}

// impl Into<(f32, f32)> for Vector2 {
//     /// Returns the [`Vector2`] as a tuple struct, where 0 = vec.x and 1 = vec.y
//     fn into(self) -> (f32, f32) {
//         (self.x, self.y)
//     }
// }

impl From<(f32, f32)> for Vector2 {
    fn from((x, y): (f32, f32)) -> Self {
        Vector2::new(x, y)
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

    fn type_name(&self) -> &str {
        self.get_represented_type_info()
            .map(|info| info.type_path())
            .unwrap_or_else(|| self.reflect_type_path())
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
