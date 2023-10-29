use serde::{Serialize, Deserialize};

use crate::bean_types::vector::Vector2;

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
pub struct Transform {
    pub position: Vector2,
    pub velocity: Vector2,
}

#[allow(dead_code)]
impl Transform {
    pub fn apply_force(&mut self, force: &Vector2) {
        self.velocity.translate_self(force, &1.0);
        todo!();
    }

    pub fn new() -> Self {
        Self {
            position: Vector2 {
                x: 0.0,
                y: 0.0,
            },
            velocity: Vector2 {
                x: 0.0,
                y: 0.0,
            }
        }
    }
}
