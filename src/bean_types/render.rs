use crate::{bean::Bean, GameInfo};
use coffee::{graphics::{Image, Window, Frame, Quad, IntoQuad}, Timer};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Renderer {
    pub dependencies: Vec<Box<dyn Bean>>,
    pub module_name: String,
    pub path: String,
    pub quad: Que,
}

#[typetag::serde]
impl Bean for Renderer {
    fn return_dependencies(&mut self) -> &mut Vec<Box<dyn Bean>> {
        &mut self.dependencies
    }
    
    #[allow(unused_variables)]
    fn init(&mut self, game_info: &GameInfo) {
        
    }

    #[allow(unused_variables)]
    fn update(&mut self, game_info: &GameInfo) {
        
    }
    
    #[allow(unused_variables)]
    fn draw(&self, game_info: &GameInfo, frame: &mut Frame, timer: &Timer) {
        let img = match game_info.assets.get_asset(&self.module_name, &self.path) {
            None => return,
            Some(img) => img,
        };

        let mut targ = frame.as_target();

        img.draw(self.quad, &mut targ);
        
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Que;

impl IntoQuad for Que {
    fn into_quad(self, x_unit: f32, y_unit: f32) -> Quad {
        todo!()
    }
}