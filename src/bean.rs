use coffee::{
    graphics::{Frame, Window},
    Timer,
};
use serde::{self, Deserialize, Serialize};

use crate::GameInfo;

pub trait Script {
    fn init(&self) {}
    fn update(&self) {}
}

#[typetag::serde(tag = "type")]
pub trait Bean {
    /// If your bean depends on other child beans to operate, then feed them in through here. Otherwise, it's fine to return an empty vec
    fn return_dependencies(&mut self) -> &mut Vec<Box<dyn Bean>>;

    /// Calls all of the initiation methods on a bean to reduce boilerplate. Override to change the functionality and remove unneeded calls on your per case basis to improve performance.
    fn _init_calls(&mut self, window: &Window) {
        self.init();

        for dep in self.return_dependencies() {
            dep.init();

            dep._init_calls(window);

            dep.ready();
        }

        self.ready();
    }

    /// Calls all of the update methods on a bean to reduce boilerplate. Override to change the functionality and remove unneeded calls on your per case basis to improve performance.
    fn _update_calls(&mut self, window: &Window, game_info: &GameInfo) {
        self.early_update();

        for dep in self.return_dependencies() {
            dep._update_calls(window, &game_info);
        }

        self.update(&game_info);
    }

    /// Calls all of the draw methods on a bean to reduce boilerplate. Override to change functionality and remove unneeded calls on your per case basis to improve performance.
    fn _draw_calls(&mut self, frame: &mut Frame, timer: &Timer) {
        for bean in self.return_dependencies() {
            bean._draw_calls(frame, timer);
        }

        self.draw(frame, timer)
    }

    /// Runs once when the bean enters the scope, will be called before all of it's scripts are finished
    fn init(&self) {}

    /// Runs once after the bean enters the scope, but unlike init(), it will only be called after all the children have run their init and ready functions
    fn ready(&self) {}

    /// Will be called once per frame, but is called before all children have run update() and earlyUpdate()
    fn early_update(&self) {}

    /// Will be called once per frame, is called after all children have run update()
    #[allow(unused_variables)]
    fn update(&mut self, game_info: &GameInfo) {}

    fn draw(&self, _frame: &mut Frame, _timer: &Timer) {}

    fn debug(&self, _frame: &mut Frame, _timer: &Timer) {}
}

#[derive(Serialize, Deserialize)]
struct MinBean {
    pub dependencies: Vec<Box<dyn Bean>>,
}

#[typetag::serde]
impl Bean for MinBean {
    fn return_dependencies(&mut self) -> &mut Vec<Box<dyn Bean>> {
        &mut self.dependencies
    }
}
