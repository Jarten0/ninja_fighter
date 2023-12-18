use super::action::KeyStatus;
use ggez::input::keyboard::KeyCode;
pub struct Key {
    pub kycode: KeyCode,
    pub name: &'static str,
    pub status: KeyStatus,
}

impl Key {
    pub fn new(name: &'static str, keycode: KeyCode) -> Self {
        Self {
            kycode: keycode,
            name,
            status: Default::default(),
        }
    }
}

impl Default for Key {
    fn default() -> Self {
        Self {
            kycode: KeyCode::X,
            name: "None",
            status: Default::default(),
        }
    }
}
