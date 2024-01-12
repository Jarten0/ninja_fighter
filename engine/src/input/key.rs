pub mod input_hashmap;
pub mod keycode_converter;
pub mod stringcode;

use self::keycode_converter::KeycodeType;

use super::action::KeyStatus;
use ggez::input::keyboard::KeyCode;
use stringcode::StringifiableKeyCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Key {
    pub keycode: StringifiableKeyCode,
    pub name: &'static str,
    pub status: KeyStatus,
    pub event_occured: bool,
}

impl Key {
    pub(crate) fn new(name: &'static str, keycode: StringifiableKeyCode) -> Self {
        Self {
            keycode,
            name,
            status: Default::default(),
            event_occured: false,
        }
    }

    pub(crate) fn update(&mut self, is_held: bool) {
        self.status = match is_held {
            true => match self.status {
                KeyStatus::Pressed => KeyStatus::Held(2),
                KeyStatus::Held(time_held) => KeyStatus::Held(time_held + 1),
                KeyStatus::Released => KeyStatus::Pressed,
                KeyStatus::Idle(_) => KeyStatus::Pressed,
            },
            false => match self.status {
                KeyStatus::Pressed => KeyStatus::Released,
                KeyStatus::Held(_) => KeyStatus::Released,
                KeyStatus::Released => KeyStatus::Idle(2),
                KeyStatus::Idle(time_released) => KeyStatus::Idle(time_released + 1),
            },
        }
    }
}

impl Default for Key {
    fn default() -> Self {
        Self {
            keycode: StringifiableKeyCode {
                0: KeycodeType::Keyboard(KeyCode::Z),
            },
            name: "None",
            status: Default::default(),
            event_occured: false,
        }
    }
}

impl ToString for Key {
    fn to_string(&self) -> String {
        let mut output = String::new();
        output.push_str(&self.keycode.to_string());

        output
    }
}
