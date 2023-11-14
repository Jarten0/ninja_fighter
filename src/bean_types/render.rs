use crate::{bean::Bean, GameInfo, math::vector::Vector2};
use coffee::{graphics::{self, Rectangle, Point}, Timer};
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
        self.quad = Que {
            position:Vector2::zero(), 
            size: (100.0, 100.0) };
    }

    #[allow(unused_variables)]
    fn update(&mut self, game_info: &GameInfo) {
        
    }
    
    #[allow(unused_variables)]
    fn draw(&self, game_info: &GameInfo, frame: &mut graphics::Frame, timer: &Timer) {
        let img = match game_info.assets.get_asset(&self.module_name, &self.path) {
            None => return,
            Some(img) => img,
        };

        let mut targ = frame.as_target();

        graphics::Image::draw(&img, self.quad.clone(), &mut targ);
        
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Que {
    pub position: Vector2,
    pub size: (f32, f32),
}

impl graphics::IntoQuad for Que {
    fn into_quad(self, _x_unit: f32, _y_unit: f32) -> graphics::Quad {
        graphics::Quad { 
            source: Rectangle { 
                x: self.position.x, 
                y: self.position.y, 
                width: self.size.0, 
                height: self.size.1, 
            }, 
            position: Point::new(0.0, 0.0),
            size: (0.0, 0.0)
        }
    }
}