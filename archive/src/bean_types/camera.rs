use crate::{bean::Bean, GameInfo};
use coffee::graphics::Window;
use serde::{Deserialize, Serialize};

use super::transform::Transform;

#[derive(Serialize, Deserialize)]
struct Camera {
    pub dpe: Vec<Box<dyn Bean>>,
}

#[allow(unused_variables)]
#[typetag::serde]
impl Bean for Camera {
    fn return_dependencies(&mut self) -> &mut Vec<Box<dyn Bean>> {
        &mut self.dpe
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        let mut dependencies: Vec<Box<dyn Bean>> = Vec::new();

        let transform: Transform = crate::bean_types::transform::Transform::new();

        dependencies.push(Box::new(transform));

        Self { dpe: dependencies }
    }

    fn init(&mut self, game_info: &mut GameInfo, window: &Window) {}

    fn ready(&mut self, game_info: &mut GameInfo, window: &Window) {}

    fn update(&mut self, game_info: &GameInfo) {}
}
