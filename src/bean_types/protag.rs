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
    fn new() -> Self {
        let mut dependencies: Vec<Box<dyn Bean>> = Vec::new();

        let transf = Transform::new();

        let mut rend = Renderer::new();
        rend.set_path(&String::from("Peep"), &String::from("Pap"));
        rend.set_size(100.0, 100.0);

        dependencies.push(Box::new(transf));
        dependencies.push(Box::new(rend));
        println!("Created the protag!");
        Self { dependencies }
    }

    fn return_dependencies(&mut self) -> &mut Vec<Box<dyn Bean>> {
        &mut self.dependencies
    }

    #[allow(unused_variables)]
    fn ready(&mut self, game_info: &GameInfo) {
        println!("Ran the funny thingy!")
    }

    #[allow(unused_variables)]
    fn update(&mut self, game_info: &GameInfo) {}
}
