use crate::{bean::Bean, math::vector::Vector2, GameInfo};
use coffee::{
    graphics::{self, Point, Rectangle},
    Timer,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Renderer {
    pub dependencies: Vec<Box<dyn Bean>>,
    module_name: Option<String>,
    path: Option<String>,
    quad: Que,
}

impl Renderer {
    pub fn set_path(&mut self, module: &String, path: &String) {
        self.path = Some(path.to_owned());
        self.module_name = Some(module.to_owned());
    }

    pub fn set_quad(&mut self, quad: Que) {
        self.quad = quad
    }

    pub fn set_pos(&mut self, position: Vector2) {
        self.quad.position = position;
    }

    pub fn set_size(&mut self, x: f32, y: f32) {
        self.quad.size = (x, y);
    }

    pub fn get_quad(&self) -> &Que {
        &self.quad
    }

    pub fn get_path(&self) -> Option<String> {
        self.path.to_owned()
    }

    pub fn get_module(&self) -> Option<String> {
        self.module_name.to_owned()
    }
}

#[typetag::serde]
impl Bean for Renderer {
    fn new() -> Self {
        Self {
            dependencies: Vec::new(),
            module_name: None,
            path: None,
            quad: Que {
                position: Vector2::zero(),
                size: (1.0, 1.0),
            },
        }
    }

    fn return_dependencies(&mut self) -> &mut Vec<Box<dyn Bean>> {
        &mut self.dependencies
    }

    #[allow(unused_variables)]
    fn init(&mut self, game_info: &GameInfo) {
        self.quad = Que {
            position: Vector2::zero(),
            size: (100.0, 100.0),
        };
    }

    #[allow(unused_variables)]
    fn update(&mut self, game_info: &GameInfo) {}

    #[allow(unused_variables)]
    fn draw(&self, game_info: &GameInfo, frame: &mut graphics::Frame, timer: &Timer) {
        let img = match game_info.assets.get_asset(
            match &self.module_name {
                None => return (),
                Some(module) => module,
            },
            match &self.path {
                None => return (),
                Some(path) => path,
            },
        ) {
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
            size: (0.0, 0.0),
        }
    }
}
