use crate::bean_types::transform::Transform;
use crate::{bean::Bean, GameRoot};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Protag {
    pub dependencies: Vec<Box<dyn Bean>>,
}

impl Default for Protag {
    fn default() -> Self {
        let mut dependencies: Vec<Box<dyn Bean>> = Vec::new();
        dependencies.push(Box::new(Transform::new()));
        Self { dependencies }
    }
}

#[typetag::serde]
impl Bean for Protag {
    fn return_dependencies(&mut self) -> &mut Vec<Box<dyn Bean>> {
        &mut self.dependencies
    }

    fn ready(&self, _game_root: &GameRoot) {}

    fn update(&mut self, _game_root: &GameRoot) {}
}
