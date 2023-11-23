pub mod assets;
pub mod bean;
pub mod bean_types;
pub mod cup;
pub mod math;

use std::collections::HashMap;

use assets::Assets;
use bean_types::render::RendererTask;
use coffee::input::keyboard::KeyCode;
use coffee::input::KeyboardAndMouse;
use coffee::load::loading_screen::ProgressBar;
use cup::{BeanGrinder, Cup};

use coffee::graphics::{Color, Frame, Image, Window, WindowSettings};
use coffee::load::Task;
use coffee::{Game, Result, Timer};

fn main() -> Result<()> {
    GameRoot::run(WindowSettings {
        title: String::from("Ninja Fighter [PROTOTYPE]"),
        size: (100 * 16, 100 * 9),
        resizable: true,
        fullscreen: false,
        maximized: false,
    })
}

#[allow(dead_code)]
pub struct GameRoot {
    pub cup: Cup,
    pub game_info: GameInfo,
    pub load_tasks: Vec<RendererTask>,
}

pub struct GameInfo {
    pub assets: Assets,
    pub delta: f32,
}

impl Game for GameRoot {
    type Input = KeyboardAndMouse;
    type LoadingScreen = ProgressBar; // No loading screen

    const TICKS_PER_SECOND: u16 = 60;
    const DEBUG_KEY: Option<KeyCode> = Some(KeyCode::F12);

    fn load(_window: &Window) -> Task<GameRoot> {
        let assets: Assets = Assets {
            internal_assets: HashMap::new(),
        };

        Image::load(String::from("ee"));

        let mut cup: Cup = BeanGrinder::brew_default_cup();
        let delta: f32 = 1.0 / GameRoot::TICKS_PER_SECOND as f32;
        let mut load_tasks: Vec<RendererTask> = Vec::new();
        let mut game_info: GameInfo = GameInfo { assets, delta };

        for bn in &mut cup.beans {
            bn._init_calls(&mut game_info, _window)
        }

        for bn in &mut cup.beans_and_dependencies {
            match bn.load() {
                None => (),
                Some(mut tasks_from_bean) => load_tasks.append(&mut tasks_from_bean),
            }
        }

        Task::succeed(move || GameRoot {
            cup,
            game_info,
            load_tasks,
        })
    }

    fn draw(&mut self, frame: &mut Frame, timer: &Timer) {
        frame.clear(Color::WHITE);

        for bean in self.cup.pour_beans() {
            bean._draw_calls(&mut self.game_info, frame, timer);
        }
    }

    fn interact(&mut self, _input: &mut Self::Input, window: &mut Window) {
        for i in 0..self.load_tasks.len() {
            let renderer_task = self.load_tasks.remove(i);
            match renderer_task.task.run(window.gpu()) {
                Ok(image) => self.game_info.assets.new_asset(
                    &renderer_task.module,
                    &renderer_task.path,
                    image,
                ),
                Err(_) => todo!(),
            };
        }
    }

    fn update(&mut self, window: &Window) {
        for bean in &mut self.cup.beans {
            bean._update_calls(&mut self.game_info, window);
        }
    }

    fn cursor_icon(&self) -> coffee::graphics::CursorIcon {
        coffee::graphics::CursorIcon::Default
    }

    fn debug(&self, _input: &Self::Input, frame: &mut Frame<'_>, debug: &mut coffee::Debug) {
        debug.draw(frame);
    }

    fn on_close_request(&mut self) -> bool {
        true
    }

    fn is_finished(&self) -> bool {
        false
    }
}
