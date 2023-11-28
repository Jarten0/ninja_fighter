use std::path::PathBuf;

use crate::bean_types::transform::Transform;
use crate::{bean::Bean, GameInfo};
use coffee::graphics::Window;
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
    fn new() -> Self {
        let mut dependencies: Vec<Box<dyn Bean>> = Vec::new();

        let transf = Transform::new();

        let mut rend = Renderer::new();
        rend.set_path(
            &String::from("Peep"),
            PathBuf::from("C:\\Users\\Markian\\Coffee\\ninja_fighter\\assets\\protag_texture.png")
                .into(),
        );
        rend.set_size(128.0, 128.0);

        dependencies.push(Box::new(transf));
        dependencies.push(Box::new(rend));
        Self { dependencies }
    }

    fn return_dependencies(&mut self) -> &mut Vec<Box<dyn Bean>> {
        &mut self.dependencies
    }

    #[allow(unused_variables)]
    fn ready(&mut self, game_info: &mut GameInfo, _window: &Window) {}

    #[allow(unused_variables)]
    fn update(&mut self, game_info: &GameInfo) {}
}
