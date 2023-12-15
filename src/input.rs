use crate::space::Vector2;
use ggez::input::keyboard::KeyCode;
use std::collections::HashMap;

pub struct MouseKeyInput {
    pub mouse_pos: Vector2,
    pub keylist: HashMap<KeyCode, Key>,
}

impl MouseKeyInput {
    //If Err is thrown, it's probably because I haven't yet implemented the requested key.
    pub fn key_just_pressed(&mut self, key_id: KeyCode) -> Result<bool, &str> {
        match self.keylist.get(&key_id) {
            Some(key) => Ok(key.is_just_pressed),
            None => Err("Invalid KeyMappingID"),
        }
    }
    pub fn key_pressed(&mut self, key_id: KeyCode) -> Result<bool, &str> {
        match self.keylist.get(&key_id) {
            Some(key) => Ok(key.is_held),
            None => Err("Invalid KeyMappingID"),
        }
    }
    pub fn key_just_released(&mut self, key_id: KeyCode) -> Result<bool, &str> {
        match self.keylist.get(&key_id) {
            Some(key) => Ok(key.is_just_released),
            None => Err("Invalid KeyMappingID"),
        }
    }
}

pub struct Key {
    kycode: KeyCode,
    key_name: &'static str,
    is_just_pressed: bool,
    is_held: bool,
    is_just_released: bool,
}

pub struct MouseKeys {
    key_list: HashMap<&'static str, Key>,
}

impl Key {
    pub fn new(name: &'static str, keycode: KeyCode) -> Self {
        Self {
            kycode: keycode,
            key_name: name,
            is_just_pressed: false,
            is_held: false,
            is_just_released: false,
        }
    }
}

impl Default for Key {
    fn default() -> Self {
        Self {
            kycode: KeyCode::X,
            key_name: "None",
            is_just_pressed: false,
            is_held: false,
            is_just_released: false,
        }
    }
}
