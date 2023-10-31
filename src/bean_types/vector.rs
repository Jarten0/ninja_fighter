use serde::{Deserialize, Serialize};

//1 unit == 1 meter
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

#[allow(dead_code)]
impl Vector2 {
    pub fn translate(vector: &Vector2, translation: &Vector2, t: &f32) -> Vector2 {
        Vector2 {
            x: vector.x + translation.x * t,
            y: vector.y + translation.y * t,
        }
    }

    pub fn translate_self(&mut self, translation: &Vector2, t: &f32) -> &mut Self {
        self.x = self.x + translation.x * t;
        self.y = self.y + translation.y * t;
        return self;
    }

    pub fn set(&mut self, vector: &Vector2) {
        *self = *vector;
    }

    pub fn new() -> Vector2 {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn new_xy(x: f32, y: f32) -> Vector2 {
        Self { x, y }
    }
}
