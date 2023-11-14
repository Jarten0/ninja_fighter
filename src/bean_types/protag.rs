use crate::bean_types::transform::Transform;
use crate::{bean::Bean, GameInfo};
use serde::{Deserialize, Serialize};

use super::render::Renderer;

#[derive(Serialize, Deserialize)]
pub struct Protag {
    pub dependencies: Vec<Box<dyn Bean>>,
}

impl Default for Protag {
    fn default() -> Self {
        let mut dependencies: Vec<Box<dyn Bean>> = Vec::new();
        dependencies.push(Box::new(Transform::new()));
        dependencies.push(Box::new(Renderer::new()));
        Self { dependencies }
    }
}

#[typetag::serde]
impl Bean for Protag {
    fn return_dependencies(&mut self) -> &mut Vec<Box<dyn Bean>> {
        &mut self.dependencies
    }

    #[allow(unused_variables)]
    fn ready(&self, game_info: &GameInfo) {}

    #[allow(unused_variables)]
    fn update(&mut self, game_info: &GameInfo) {}
}
