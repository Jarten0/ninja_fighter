use crate::{bean::Bean, GameRoot};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Renderer {
    pub dependencies: Vec<Box<dyn Bean>>,
}

#[typetag::serde]
impl Bean for Renderer {
    fn return_dependencies(&self) -> &Vec<Box<dyn Bean>> {
        &self.dependencies
    }

    fn update(&self, _game_root: &GameRoot) {}
}
