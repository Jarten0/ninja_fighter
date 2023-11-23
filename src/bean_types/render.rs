use core::panic;
use std::path::{Path, PathBuf};

use crate::{bean::Bean, math::vector::Vector2, GameInfo};
use coffee::{
    graphics::{self, Frame, Image, Point, Rectangle, Window},
    load::Task,
    Timer,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Renderer {
    pub dependencies: Vec<Box<dyn Bean>>,
    module_name: Option<String>,
    path: Option<Box<PathBuf>>,
    quad: Que,

    ready: bool,
}

impl Renderer {
    pub fn set(
        &mut self,
        game_info: &mut GameInfo,
        module: String,
        image: Image,
        path: Option<Box<PathBuf>>,
    ) {
        game_info.assets.new_module(&module);

        match path {
            Some(path) => game_info.assets.new_asset(&module, &path.clone(), image),
            None => None,
        };
    }

    pub fn set_path(&mut self, module: &String, path: Box<PathBuf>) {
        self.path = Some(path);
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

    pub fn get_path(&self) -> &Option<Box<PathBuf>> {
        &self.path
    }

    pub fn get_module(&self) -> Option<String> {
        self.module_name.to_owned()
    }

    fn prep_self(&mut self, game_info: &mut GameInfo, window: &mut Frame) {
        let path = match self.path.to_owned() {
            None => Box::new(PathBuf::from("assets/default/default_texture.png")),
            Some(path) => path,
        };

        let module = match self.module_name.to_owned() {
            None => String::from("Teehee"),
            Some(path) => path,
        };

        let image = match Image::new::<&Path>(window.gpu(), path.as_path()) {
            Ok(img) => img,
            Err(..) => get_default_texture(window),
        };

        self.set(game_info, module, image, Some(path));
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
            ready: false,
        }
    }

    fn return_dependencies(&mut self) -> &mut Vec<Box<dyn Bean>> {
        &mut self.dependencies
    }

    #[allow(unused_variables)]
    fn init(&mut self, game_info: &mut GameInfo, window: &Window) {
        self.quad = Que {
            position: Vector2::zero(),
            size: (100.0, 100.0),
        };
    }

    fn load(&mut self) -> Option<Vec<RendererTask>> {
        let mut tasks: Vec<RendererTask> = Vec::new();

        let path = match self.get_path() {
            None => return None,
            Some(path) => path.to_owned(),
        };

        let module = match self.module_name.to_owned() {
            Some(module) => module,
            None => String::from("default_module"),
        };

        tasks.push(RendererTask {
            task: Image::load::<&PathBuf>(path.as_ref()),
            path: *path,
            module,
        });
        Some(tasks)
    }

    #[allow(unused_variables)]
    fn update(&mut self, game_info: &GameInfo) {}

    #[allow(unused_variables)]
    fn draw(&mut self, game_info: &mut GameInfo, frame: &mut graphics::Frame, timer: &Timer) {
        if self.ready == false {
            self.prep_self(game_info, frame);
            self.ready = true;
        }

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

        let default = get_default_texture(frame);

        let mut targ = frame.as_target();

        graphics::Image::draw(&img, self.quad.clone(), &mut targ);
        graphics::Image::draw(&default, self.quad.clone(), &mut targ);
    }
}

fn get_default_texture(frame: &mut Frame) -> Image {
    match Image::new(
        frame.gpu(),
        &Path::new("C:\\Users\\Markian\\Coffee\\ninja_fighter\\assets\\default_texture.png"),
    ) {
        Result::Ok(img) => img,
        Result::Err(err) => panic!("Failed to obtain default texture! {}", err),
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
            size: self.size,
        }
    }
}

pub struct RendererTask {
    pub task: Task<Image>,
    pub path: PathBuf,
    pub module: String,
}
