use serde::{Deserialize, Serialize};
use std::ops;

//1 unit == 1 meter
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

#[allow(dead_code)]
impl Vector2 {
    pub fn zero() -> Vector2 {
        Vector2 { x: 0.0, y: 0.0 }
    }
    pub fn one() -> Vector2 {
        Vector2 { x: 1.0, y: 1.0 }
    }
    pub fn normal() -> Vector2 {
        Vector2 {
            x: 0.70710678118,
            y: 0.70710678118,
        }
    }
    pub fn up() -> Vector2 {
        Vector2 { x: 0.0, y: 1.0 }
    }
    pub fn down() -> Vector2 {
        Vector2 { x: 0.0, y: -1.0 }
    }
    pub fn left() -> Vector2 {
        Vector2 { x: -1.0, y: 0.0 }
    }
    pub fn right() -> Vector2 {
        Vector2 { x: 1.0, y: 0.0 }
    }

    /// Translates the vector by the given amount over time.
    ///
    /// t is a multiplier for the amount of the distance you want to translate in this call.
    ///
    /// ### Examples
    /// ```
    /// let mut vector = Vector2::zero();
    /// let delta: f32 = 0.02;
    ///
    /// vector.translate(Vector2::one(), delta);
    /// ```
    pub fn translate(&mut self, translation: &Vector2, t: &f32) -> &mut Self {
        self.x = self.x + translation.x * t;
        self.y = self.y + translation.y * t;
        return self;
    }

    /// Translates the vector by the given amount.
    pub fn translate_instantaneous(&mut self, translation: &Vector2) -> &mut Self {
        self.x = self.x + translation.x;
        self.y = self.y + translation.y;
        return self;
    }

    /// Returns a scalar value representing the length of the vector.
    ///
    /// If an unusual value is found, ie if the vector is empty, this will return Vector2.ZERO
    pub fn magnitude(&self) -> f32 {
        let mag = (self.x.powf(2.0) + self.y.powf(2.0)).sqrt();
        if !mag.is_normal() {
            return 0.0;
        }
        mag
    }

    /// Returns a scalar value representing the length of the vector.
    ///
    /// Unlike Vector2.magnitude(), this will return whatever value is returned from the equation, so you may get NaN, Infinity, or a subnormal number.
    pub fn magnitude_unchecked(&self) -> f32 {
        return (self.x.powf(2.0) + self.y.powf(2.0)).sqrt();
    }

    /// Returns a vector with each component divided by the magnitude, resulting in a vector that points in a direction with a magnitude of one.
    ///
    /// To avoid instances where the magnitude is non-sensible, this will return Option<Vector2> where in the event that an unusable number is given, this function
    /// will return None.
    pub fn normalized(&self) -> Option<Vector2> {
        let mag = self.magnitude();
        if !mag.is_normal() {
            return None;
        }
        Some(Self {
            x: self.x / mag,
            y: self.y / mag,
        })
    }

    /// Sets the current vector to the vector specified
    pub fn set(&mut self, vector: &Vector2) {
        *self = *vector;
    }

    /// Creates a new zero vector
    pub fn new() -> Vector2 {
        Self { x: 0.0, y: 0.0 }
    }

    /// Creates a new vector with the x and y values provided
    pub fn new_xy(x: f32, y: f32) -> Vector2 {
        Self { x, y }
    }
}

impl ops::Add for Vector2 {
    type Output = Vector2;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Add<f32> for Vector2 {
    type Output = Vector2;
    fn add(self, rhs: f32) -> Self::Output {
        match self.normalized() {
            None => return Vector2::zero(),
            Some(norm) => norm + Vector2::normal() * rhs,
        }
    }
}

impl ops::Add<i32> for Vector2 {
    type Output = Vector2;
    fn add(self, rhs: i32) -> Self::Output {
        match self.normalized() {
            None => return Vector2::zero(),
            Some(norm) => norm + Vector2::normal() * rhs,
        }
    }
}

impl ops::Add<i16> for Vector2 {
    type Output = Vector2;
    fn add(self, rhs: i16) -> Self::Output {
        match self.normalized() {
            None => return Vector2::zero(),
            Some(norm) => norm + Vector2::normal() * rhs,
        }
    }
}

impl ops::Add<i8> for Vector2 {
    type Output = Vector2;
    fn add(self, rhs: i8) -> Self::Output {
        match self.normalized() {
            None => return Vector2::zero(),
            Some(norm) => norm + Vector2::normal() * rhs,
        }
    }
}

impl ops::Add<isize> for Vector2 {
    type Output = Vector2;
    fn add(self, rhs: isize) -> Self::Output {
        match self.normalized() {
            None => return Vector2::zero(),
            Some(norm) => norm + Vector2::normal() * rhs,
        }
    }
}

impl ops::Mul<f32> for Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl ops::Mul<i32> for Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x * rhs as f32,
            y: self.y * rhs as f32,
        }
    }
}

impl ops::Mul<i16> for Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: i16) -> Self::Output {
        Self {
            x: self.x * rhs as f32,
            y: self.y * rhs as f32,
        }
    }
}

impl ops::Mul<i8> for Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: i8) -> Self::Output {
        Self {
            x: self.x * rhs as f32,
            y: self.y * rhs as f32,
        }
    }
}

impl ops::Mul<isize> for Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: isize) -> Self::Output {
        Self {
            x: self.x * rhs as f32,
            y: self.y * rhs as f32,
        }
    }
}
