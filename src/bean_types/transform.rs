use serde::{Deserialize, Serialize};

use crate::{bean::Bean, bean_types::vector::Vector2, GameInfo};

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
pub struct Transform {
    pub position: Vector2,
    pub velocity: Vector2,
    pub dependencies: Vec<Box<dyn Bean>>,
}

#[allow(dead_code)]
impl Transform {
    pub fn apply_force(&mut self, force: &Vector2) {
        self.velocity.translate_instantaneous(force);
        todo!();
    }

    pub fn new() -> Self {
        Self {
            position: Vector2 { x: 0.0, y: 0.0 },
            velocity: Vector2 { x: 0.0, y: 0.0 },
            dependencies: Vec::new(),
        }
    }
}

#[typetag::serde]
impl Bean for Transform {
    fn return_dependencies(&mut self) -> &mut Vec<Box<dyn Bean>> {
        &mut self.dependencies
    }

    fn update(&mut self, game_info: &GameInfo) {
        self.position.translate(&self.velocity, &game_info.delta);
    }
}
