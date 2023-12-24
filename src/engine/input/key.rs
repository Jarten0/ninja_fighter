pub mod keycode_converter;
pub mod stringcode;

use self::keycode_converter::KeyTypes;

use super::action::KeyStatus;
use ggez::input::keyboard::KeyCode;
use stringcode::StringifiableKeyCode;

pub struct Key {
    pub keycode: StringifiableKeyCode,
    pub name: &'static str,
    pub status: KeyStatus,
}

impl Key {
    pub fn new(name: &'static str, keycode: StringifiableKeyCode) -> Self {
        Self {
            keycode,
            name,
            status: Default::default(),
        }
    }
}

impl Default for Key {
    fn default() -> Self {
        Self {
            keycode: StringifiableKeyCode {
                0: KeyTypes::Keyboard(KeyCode::Z),
            },
            name: "None",
            status: Default::default(),
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
