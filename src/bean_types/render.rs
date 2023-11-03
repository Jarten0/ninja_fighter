use crate::{bean::Bean, GameRoot};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Renderer {
    pub dependencies: Vec<Box<dyn Bean>>,
}

#[typetag::serde]
impl Bean for Renderer {
    fn return_dependencies(&mut self) -> &mut Vec<Box<dyn Bean>> {
        &mut self.dependencies
    }

    fn update(&mut self, _game_root: &GameRoot) {}
}
