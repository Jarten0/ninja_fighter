pub mod bean_types;
pub mod bean;
pub mod cup;

use bean_types::transform::Transform;
use cup::Cup;
use bean::{Bean, Protag};
use coffee::graphics::{Color, Frame, Window, WindowSettings};
use coffee::load::Task;
use coffee::{Game, Result, Timer};

fn main() -> Result<()> {
    MyGame::run(WindowSettings {
        title: String::from("Ninja Fighter [PROTOTYPE]"),
        size: (100 * 16, 100 * 9),
        resizable: true,
        fullscreen: false,
        maximized: false,
    })
}

#[allow(dead_code)]
struct MyGame {
    // bean_grinder: BeanGrinder,
    cup: Cup
}


impl Game for MyGame {
    type Input = (); // No input data
    type LoadingScreen = (); // No loading screen
    
    fn load(_window: &Window) -> Task<MyGame> {
        
        
        Task::succeed(|| MyGame { 
            // bean_grinder: BeanGrinder { 
                 
            // },
            cup: Cup { 
                beans: { 
                    let mut default = Vec::new(); 
                    let protag = Protag { transform: Transform::new()};
                    let boxx: Box<dyn Bean> = Box::new(protag);
                    default.push(boxx);

                    default
                } 
            }

        })
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        frame.clear(Color::WHITE);

        for bean in &self.cup.beans {
            bean.update();
        }
    }
}

