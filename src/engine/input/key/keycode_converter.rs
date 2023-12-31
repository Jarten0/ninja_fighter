//! Provides several conversion tables for converting from strings and keycodes
//!

use std::str::FromStr;
use std::{collections::HashMap, sync::OnceLock};

use enum_iterator::Sequence;
use ggez::event::{Button, MouseButton};
use ggez::input::keyboard::KeyCode;

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum KeycodeType {
    Keyboard(KeyCode),  // 161 variants
    Gamepad(Button),    // 18 variants
    Mouse(MouseButton), // 4 variants
}

impl ToString for KeycodeType {
    fn to_string(&self) -> String {
        String::from(keycode_to_str(*self).unwrap())
    }
}

impl FromStr for KeycodeType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match str_to_keycode(s) {
            Some(value) => Ok(value),
            None => Err("Keycode not found!"),
        }
    }
}

impl PartialOrd for KeycodeType {
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        todo!()
    }
}

impl Ord for KeycodeType {
    fn cmp(&self, _other: &Self) -> std::cmp::Ordering {
        todo!()
    }
}

impl Sequence for KeycodeType {
    const CARDINALITY: usize = 183;

    fn next(&self) -> Option<Self> {
        if *self == KeycodeType::Keyboard(KeyCode::Tab) {
            return Some(KeycodeType::Gamepad(Button::South));
        } else if *self == KeycodeType::Gamepad(Button::Unknown) {
            return Some(KeycodeType::Mouse(MouseButton::Left));
        } else if *self == KeycodeType::Mouse(MouseButton::Other(0)) {
            return None;
        } else {
            Some(match self {
                KeycodeType::Keyboard(keycode) => KeycodeType::Keyboard(match keycode {
                    KeyCode::Key1 => KeyCode::Key2,
                    KeyCode::Key2 => KeyCode::Key3,
                    KeyCode::Key3 => KeyCode::Key4,
                    KeyCode::Key4 => KeyCode::Key5,
                    KeyCode::Key5 => KeyCode::Key6,
                    KeyCode::Key6 => KeyCode::Key7,
                    KeyCode::Key7 => KeyCode::Key8,
                    KeyCode::Key8 => KeyCode::Key9,
                    KeyCode::Key9 => KeyCode::Key0,
                    KeyCode::Key0 => KeyCode::A,
                    KeyCode::A => KeyCode::B,
                    KeyCode::B => KeyCode::C,
                    KeyCode::C => KeyCode::D,
                    KeyCode::D => KeyCode::E,
                    KeyCode::E => KeyCode::F,
                    KeyCode::F => KeyCode::G,
                    KeyCode::G => KeyCode::H,
                    KeyCode::H => KeyCode::I,
                    KeyCode::I => KeyCode::J,
                    KeyCode::J => KeyCode::K,
                    KeyCode::K => KeyCode::L,
                    KeyCode::L => KeyCode::M,
                    KeyCode::M => KeyCode::N,
                    KeyCode::N => KeyCode::O,
                    KeyCode::O => KeyCode::P,
                    KeyCode::P => KeyCode::Q,
                    KeyCode::Q => KeyCode::R,
                    KeyCode::R => KeyCode::S,
                    KeyCode::S => KeyCode::T,
                    KeyCode::T => KeyCode::U,
                    KeyCode::U => KeyCode::V,
                    KeyCode::V => KeyCode::W,
                    KeyCode::W => KeyCode::X,
                    KeyCode::X => KeyCode::Y,
                    KeyCode::Y => KeyCode::Z,
                    KeyCode::Z => KeyCode::Escape,
                    KeyCode::Escape => KeyCode::F1,
                    KeyCode::F1 => KeyCode::F2,
                    KeyCode::F2 => KeyCode::F3,
                    KeyCode::F3 => KeyCode::F4,
                    KeyCode::F4 => KeyCode::F5,
                    KeyCode::F5 => KeyCode::F6,
                    KeyCode::F6 => KeyCode::F7,
                    KeyCode::F7 => KeyCode::F8,
                    KeyCode::F8 => KeyCode::F9,
                    KeyCode::F9 => KeyCode::F10,
                    KeyCode::F10 => KeyCode::F11,
                    KeyCode::F11 => KeyCode::F12,
                    KeyCode::F12 => KeyCode::F13,
                    KeyCode::F13 => KeyCode::F14,
                    KeyCode::F14 => KeyCode::F15,
                    KeyCode::F15 => KeyCode::F16,
                    KeyCode::F16 => KeyCode::F17,
                    KeyCode::F17 => KeyCode::F18,
                    KeyCode::F18 => KeyCode::F19,
                    KeyCode::F19 => KeyCode::F20,
                    KeyCode::F20 => KeyCode::F21,
                    KeyCode::F21 => KeyCode::F22,
                    KeyCode::F22 => KeyCode::F23,
                    KeyCode::F23 => KeyCode::F24,
                    KeyCode::F24 => KeyCode::Insert,
                    KeyCode::Insert => KeyCode::Home,
                    KeyCode::Home => KeyCode::Delete,
                    KeyCode::Delete => KeyCode::End,
                    KeyCode::End => KeyCode::PageDown,
                    KeyCode::PageDown => KeyCode::PageUp,
                    KeyCode::PageUp => KeyCode::Left,
                    KeyCode::Left => KeyCode::Up,
                    KeyCode::Up => KeyCode::Right,
                    KeyCode::Right => KeyCode::Down,
                    KeyCode::Down => KeyCode::Back,
                    KeyCode::Back => KeyCode::Return,
                    KeyCode::Return => KeyCode::Space,
                    KeyCode::Space => KeyCode::Compose,
                    KeyCode::Compose => KeyCode::Caret,
                    KeyCode::Caret => KeyCode::Numlock,
                    KeyCode::Numlock => KeyCode::Numpad0,
                    KeyCode::Numpad0 => KeyCode::Numpad1,
                    KeyCode::Numpad1 => KeyCode::Numpad2,
                    KeyCode::Numpad2 => KeyCode::Numpad3,
                    KeyCode::Numpad3 => KeyCode::Numpad4,
                    KeyCode::Numpad4 => KeyCode::Numpad5,
                    KeyCode::Numpad5 => KeyCode::Numpad6,
                    KeyCode::Numpad6 => KeyCode::Numpad7,
                    KeyCode::Numpad7 => KeyCode::Numpad8,
                    KeyCode::Numpad8 => KeyCode::Numpad9,
                    KeyCode::Numpad9 => KeyCode::NumpadAdd,
                    KeyCode::NumpadAdd => KeyCode::NumpadDivide,
                    KeyCode::NumpadDivide => KeyCode::NumpadDecimal,
                    KeyCode::NumpadDecimal => KeyCode::NumpadComma,
                    KeyCode::NumpadComma => KeyCode::NumpadEnter,
                    KeyCode::NumpadEnter => KeyCode::NumpadEquals,
                    KeyCode::NumpadEquals => KeyCode::NumpadMultiply,
                    KeyCode::NumpadMultiply => KeyCode::NumpadSubtract,
                    KeyCode::NumpadSubtract => KeyCode::Backslash,
                    KeyCode::Backslash => KeyCode::Equals,
                    KeyCode::Equals => KeyCode::LAlt,
                    KeyCode::LAlt => KeyCode::LBracket,
                    KeyCode::LBracket => KeyCode::LControl,
                    KeyCode::LControl => KeyCode::LShift,
                    KeyCode::LShift => KeyCode::LWin,
                    KeyCode::LWin => KeyCode::Minus,
                    KeyCode::Minus => KeyCode::Period,
                    KeyCode::Period => KeyCode::RAlt,
                    KeyCode::RAlt => KeyCode::RShift,
                    KeyCode::RShift => KeyCode::RWin,
                    KeyCode::RWin => KeyCode::Semicolon,
                    KeyCode::Semicolon => KeyCode::Slash,
                    KeyCode::Slash => KeyCode::Tab,
                    _ => unimplemented!(),
                }),
                KeycodeType::Gamepad(buttoncode) => KeycodeType::Gamepad(match buttoncode {
                    Button::South => Button::East,
                    Button::East => Button::North,
                    Button::North => Button::West,
                    Button::West => Button::C,
                    Button::C => Button::Z,
                    Button::Z => Button::LeftTrigger,
                    Button::LeftTrigger => Button::LeftTrigger2,
                    Button::LeftTrigger2 => Button::RightTrigger,
                    Button::RightTrigger => Button::RightTrigger2,
                    Button::RightTrigger2 => Button::Select,
                    Button::Select => Button::Start,
                    Button::Start => Button::Mode,
                    Button::Mode => Button::LeftThumb,
                    Button::LeftThumb => Button::RightThumb,
                    Button::RightThumb => Button::DPadUp,
                    Button::DPadUp => Button::DPadDown,
                    Button::DPadDown => Button::DPadLeft,
                    Button::DPadLeft => Button::DPadRight,
                    Button::DPadRight => Button::Unknown,
                    Button::Unknown => unreachable!(),
                }),
                KeycodeType::Mouse(mousekeycode) => KeycodeType::Mouse(match mousekeycode {
                    MouseButton::Left => MouseButton::Right,
                    MouseButton::Right => MouseButton::Middle,
                    MouseButton::Middle => MouseButton::Other(0),
                    MouseButton::Other(_othermousekeycode) => unimplemented!(
                        "Mouse keycode {} is currently unimplemented",
                        _othermousekeycode
                    ),
                }),
            })
        }
    }

    fn previous(&self) -> Option<Self> {
        todo!()
    }

    fn first() -> Option<Self> {
        Some(KeycodeType::Keyboard(KeyCode::A))
    }

    fn last() -> Option<Self> {
        Some(KeycodeType::Mouse(MouseButton::Other(0)))
    }
}

/// Used by [`str_to_keycode`] to convert str's to keycodes via the hashmap returned
///
/// Since it uses a [`OnceLock`], it won't take any extra performance after initialization.
/// Only on the first call will it create the hashmap.
fn const_keytypes_hashmap() -> &'static HashMap<&'static str, KeycodeType> {
    static HASHMAP: OnceLock<HashMap<&str, KeycodeType>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        let mut hash_map = HashMap::new();

        {
            // Numbers
            hash_map.insert("1", KeycodeType::Keyboard(KeyCode::Key1));
            hash_map.insert("2", KeycodeType::Keyboard(KeyCode::Key2));
            hash_map.insert("3", KeycodeType::Keyboard(KeyCode::Key3));
            hash_map.insert("4", KeycodeType::Keyboard(KeyCode::Key4));
            hash_map.insert("5", KeycodeType::Keyboard(KeyCode::Key5));
            hash_map.insert("6", KeycodeType::Keyboard(KeyCode::Key6));
            hash_map.insert("7", KeycodeType::Keyboard(KeyCode::Key7));
            hash_map.insert("8", KeycodeType::Keyboard(KeyCode::Key8));
            hash_map.insert("9", KeycodeType::Keyboard(KeyCode::Key9));
            hash_map.insert("0", KeycodeType::Keyboard(KeyCode::Key0));
            // Letters
            hash_map.insert("a", KeycodeType::Keyboard(KeyCode::A));
            hash_map.insert("b", KeycodeType::Keyboard(KeyCode::B));
            hash_map.insert("c", KeycodeType::Keyboard(KeyCode::C));
            hash_map.insert("d", KeycodeType::Keyboard(KeyCode::D));
            hash_map.insert("e", KeycodeType::Keyboard(KeyCode::E));
            hash_map.insert("f", KeycodeType::Keyboard(KeyCode::F));
            hash_map.insert("g", KeycodeType::Keyboard(KeyCode::G));
            hash_map.insert("h", KeycodeType::Keyboard(KeyCode::H));
            hash_map.insert("i", KeycodeType::Keyboard(KeyCode::I));
            hash_map.insert("j", KeycodeType::Keyboard(KeyCode::J));
            hash_map.insert("k", KeycodeType::Keyboard(KeyCode::K));
            hash_map.insert("l", KeycodeType::Keyboard(KeyCode::L));
            hash_map.insert("m", KeycodeType::Keyboard(KeyCode::M));
            hash_map.insert("n", KeycodeType::Keyboard(KeyCode::N));
            hash_map.insert("o", KeycodeType::Keyboard(KeyCode::O));
            hash_map.insert("p", KeycodeType::Keyboard(KeyCode::P));
            hash_map.insert("q", KeycodeType::Keyboard(KeyCode::Q));
            hash_map.insert("r", KeycodeType::Keyboard(KeyCode::R));
            hash_map.insert("s", KeycodeType::Keyboard(KeyCode::S));
            hash_map.insert("t", KeycodeType::Keyboard(KeyCode::T));
            hash_map.insert("u", KeycodeType::Keyboard(KeyCode::U));
            hash_map.insert("v", KeycodeType::Keyboard(KeyCode::V));
            hash_map.insert("w", KeycodeType::Keyboard(KeyCode::W));
            hash_map.insert("x", KeycodeType::Keyboard(KeyCode::X));
            hash_map.insert("y", KeycodeType::Keyboard(KeyCode::Y));
            hash_map.insert("y", KeycodeType::Keyboard(KeyCode::Z));
            // Top Row
            hash_map.insert("esc", KeycodeType::Keyboard(KeyCode::Escape));
            hash_map.insert("f1", KeycodeType::Keyboard(KeyCode::F1));
            hash_map.insert("f2", KeycodeType::Keyboard(KeyCode::F2));
            hash_map.insert("f3", KeycodeType::Keyboard(KeyCode::F3));
            hash_map.insert("f4", KeycodeType::Keyboard(KeyCode::F4));
            hash_map.insert("f5", KeycodeType::Keyboard(KeyCode::F5));
            hash_map.insert("f6", KeycodeType::Keyboard(KeyCode::F6));
            hash_map.insert("f7", KeycodeType::Keyboard(KeyCode::F7));
            hash_map.insert("f8", KeycodeType::Keyboard(KeyCode::F8));
            hash_map.insert("f9", KeycodeType::Keyboard(KeyCode::F9));
            hash_map.insert("f10", KeycodeType::Keyboard(KeyCode::F10));
            hash_map.insert("f11", KeycodeType::Keyboard(KeyCode::F11));
            hash_map.insert("f12", KeycodeType::Keyboard(KeyCode::F12));
            hash_map.insert("f13", KeycodeType::Keyboard(KeyCode::F13));
            hash_map.insert("f14", KeycodeType::Keyboard(KeyCode::F14));
            hash_map.insert("f15", KeycodeType::Keyboard(KeyCode::F15));
            hash_map.insert("f16", KeycodeType::Keyboard(KeyCode::F16));
            hash_map.insert("f17", KeycodeType::Keyboard(KeyCode::F17));
            hash_map.insert("f18", KeycodeType::Keyboard(KeyCode::F18));
            hash_map.insert("f19", KeycodeType::Keyboard(KeyCode::F19));
            hash_map.insert("f20", KeycodeType::Keyboard(KeyCode::F20));
            hash_map.insert("f21", KeycodeType::Keyboard(KeyCode::F21));
            hash_map.insert("f22", KeycodeType::Keyboard(KeyCode::F22));
            hash_map.insert("f23", KeycodeType::Keyboard(KeyCode::F23));
            hash_map.insert("f24", KeycodeType::Keyboard(KeyCode::F24));
            // Navigation
            hash_map.insert("insert", KeycodeType::Keyboard(KeyCode::Insert));
            hash_map.insert("home", KeycodeType::Keyboard(KeyCode::Home));
            hash_map.insert("delete", KeycodeType::Keyboard(KeyCode::Delete));
            hash_map.insert("end", KeycodeType::Keyboard(KeyCode::End));
            hash_map.insert("pagedown", KeycodeType::Keyboard(KeyCode::PageDown));
            hash_map.insert("pageup", KeycodeType::Keyboard(KeyCode::PageUp));
            hash_map.insert("left", KeycodeType::Keyboard(KeyCode::Left));
            hash_map.insert("right", KeycodeType::Keyboard(KeyCode::Up));
            hash_map.insert("up", KeycodeType::Keyboard(KeyCode::Right));
            hash_map.insert("down", KeycodeType::Keyboard(KeyCode::Down));
            // Numpad
            hash_map.insert("numlock", KeycodeType::Keyboard(KeyCode::Numlock));
            hash_map.insert("numpad0", KeycodeType::Keyboard(KeyCode::Numpad0));
            hash_map.insert("numpad1", KeycodeType::Keyboard(KeyCode::Numpad1));
            hash_map.insert("numpad2", KeycodeType::Keyboard(KeyCode::Numpad2));
            hash_map.insert("numpad3", KeycodeType::Keyboard(KeyCode::Numpad3));
            hash_map.insert("numpad4", KeycodeType::Keyboard(KeyCode::Numpad4));
            hash_map.insert("numpad5", KeycodeType::Keyboard(KeyCode::Numpad5));
            hash_map.insert("numpad6", KeycodeType::Keyboard(KeyCode::Numpad6));
            hash_map.insert("numpad7", KeycodeType::Keyboard(KeyCode::Numpad7));
            hash_map.insert("numpad8", KeycodeType::Keyboard(KeyCode::Numpad8));
            hash_map.insert("numpad9", KeycodeType::Keyboard(KeyCode::Numpad9));
            hash_map.insert("numpadadd", KeycodeType::Keyboard(KeyCode::NumpadAdd));
            hash_map.insert("numpaddiv", KeycodeType::Keyboard(KeyCode::NumpadDivide));
            hash_map.insert(
                "numpaddecimal",
                KeycodeType::Keyboard(KeyCode::NumpadDecimal),
            );
            hash_map.insert("numpadcomma", KeycodeType::Keyboard(KeyCode::NumpadComma));
            hash_map.insert("numpadenter", KeycodeType::Keyboard(KeyCode::NumpadEnter));
            hash_map.insert("numpadequals", KeycodeType::Keyboard(KeyCode::NumpadEquals));
            hash_map.insert("numpadmul", KeycodeType::Keyboard(KeyCode::NumpadMultiply));
            hash_map.insert("numpadsub", KeycodeType::Keyboard(KeyCode::NumpadSubtract));

            hash_map.insert("backspace", KeycodeType::Keyboard(KeyCode::Back));
            hash_map.insert("return", KeycodeType::Keyboard(KeyCode::Return));
            hash_map.insert("space", KeycodeType::Keyboard(KeyCode::Space));
            hash_map.insert("compose", KeycodeType::Keyboard(KeyCode::Compose));
            hash_map.insert("caret", KeycodeType::Keyboard(KeyCode::Caret));
            hash_map.insert("bslash", KeycodeType::Keyboard(KeyCode::Backslash));
            hash_map.insert("eq", KeycodeType::Keyboard(KeyCode::Equals));

            hash_map.insert("lalt", KeycodeType::Keyboard(KeyCode::LAlt));
            hash_map.insert("lctrl", KeycodeType::Keyboard(KeyCode::LControl));
            hash_map.insert("lshift", KeycodeType::Keyboard(KeyCode::LShift));
            hash_map.insert("lwin", KeycodeType::Keyboard(KeyCode::LWin));
            hash_map.insert("ralt", KeycodeType::Keyboard(KeyCode::RAlt));
            hash_map.insert("rshift", KeycodeType::Keyboard(KeyCode::RShift));
            hash_map.insert("rwin", KeycodeType::Keyboard(KeyCode::RWin));
            hash_map.insert("rctrl", KeycodeType::Keyboard(KeyCode::RControl));

            hash_map.insert("comma", KeycodeType::Keyboard(KeyCode::Comma));
            hash_map.insert("minus", KeycodeType::Keyboard(KeyCode::Minus));
            hash_map.insert("period", KeycodeType::Keyboard(KeyCode::Period));
            hash_map.insert("semicolon", KeycodeType::Keyboard(KeyCode::Semicolon));
            hash_map.insert("lbracket", KeycodeType::Keyboard(KeyCode::LBracket));
            hash_map.insert("slash", KeycodeType::Keyboard(KeyCode::Slash));
            hash_map.insert("tab", KeycodeType::Keyboard(KeyCode::Tab));

            hash_map.insert("mouse1", KeycodeType::Mouse(MouseButton::Left));
            hash_map.insert("mouse2", KeycodeType::Mouse(MouseButton::Right));
            hash_map.insert("mouse3", KeycodeType::Mouse(MouseButton::Middle));
            hash_map.insert("mouse0", KeycodeType::Mouse(MouseButton::Other(0)));
            // TODO: Set mouse key values for mouse buttons
            hash_map.insert("mouse4", KeycodeType::Mouse(MouseButton::Other(0)));
            hash_map.insert("mouse5", KeycodeType::Mouse(MouseButton::Other(0)));

            hash_map.insert("gamepad_south", KeycodeType::Gamepad(Button::South));
            hash_map.insert("gamepad_east", KeycodeType::Gamepad(Button::East));
            hash_map.insert("gamepad_north", KeycodeType::Gamepad(Button::North));
            hash_map.insert("gamepad_west", KeycodeType::Gamepad(Button::West));
            hash_map.insert("gamepad_c", KeycodeType::Gamepad(Button::C));
            hash_map.insert("gamepad_z", KeycodeType::Gamepad(Button::Z));
            hash_map.insert("gamepad_l1", KeycodeType::Gamepad(Button::LeftTrigger));
            hash_map.insert("gamepad_l2", KeycodeType::Gamepad(Button::LeftTrigger2));
            hash_map.insert("gamepad_r1", KeycodeType::Gamepad(Button::RightTrigger));
            hash_map.insert("gamepad_r2", KeycodeType::Gamepad(Button::RightTrigger2));
            hash_map.insert("gamepad_select", KeycodeType::Gamepad(Button::Select));
            hash_map.insert("gamepad_start", KeycodeType::Gamepad(Button::Start));
            hash_map.insert("gamepad_mode", KeycodeType::Gamepad(Button::Mode));
            hash_map.insert("gamepad_l3", KeycodeType::Gamepad(Button::LeftThumb));
            hash_map.insert("gamepad_r3", KeycodeType::Gamepad(Button::RightThumb));
            hash_map.insert("gamepad_dup", KeycodeType::Gamepad(Button::DPadUp));
            hash_map.insert("gamepad_ddown", KeycodeType::Gamepad(Button::DPadDown));
            hash_map.insert("gamepad_dleft", KeycodeType::Gamepad(Button::DPadLeft));
            hash_map.insert("gamepad_dright", KeycodeType::Gamepad(Button::DPadRight));
            hash_map.insert("gamepad_unknown", KeycodeType::Gamepad(Button::Unknown));
        }

        hash_map
    })
}

/// Returns a [`KeyTypes`] equivilant to the inputted `&str`, returns [`None`] if the keycode is not found.
pub fn str_to_keycode(str_ptr: &str) -> Option<KeycodeType> {
    Some(const_keytypes_hashmap().get(str_ptr)?.to_owned())
}

/// Returns a &[`str`] equivilant of the keycode that can be displayed or serialized, and can later turned back into a keycode using [`str_to_keycode`]
///
/// Returns [`Ok`] if the keycode has a string equivilant, returns [`Err`] if the keycode currently has not been implemented.
pub fn keycode_to_str(key_type: KeycodeType) -> Result<&'static str, &'static str> {
    let str_ptr = match key_type {
        KeycodeType::Keyboard(keycode) => match keycode {
            // Numbers
            KeyCode::Key1 => "1",
            KeyCode::Key2 => "2",
            KeyCode::Key3 => "3",
            KeyCode::Key4 => "4",
            KeyCode::Key5 => "5",
            KeyCode::Key6 => "6",
            KeyCode::Key7 => "7",
            KeyCode::Key8 => "8",
            KeyCode::Key9 => "9",
            KeyCode::Key0 => "0",
            // Letters
            KeyCode::A => "a",
            KeyCode::B => "b",
            KeyCode::C => "c",
            KeyCode::D => "d",
            KeyCode::E => "e",
            KeyCode::F => "f",
            KeyCode::G => "g",
            KeyCode::H => "h",
            KeyCode::I => "i",
            KeyCode::J => "j",
            KeyCode::K => "k",
            KeyCode::L => "l",
            KeyCode::M => "m",
            KeyCode::N => "n",
            KeyCode::O => "o",
            KeyCode::P => "p",
            KeyCode::Q => "q",
            KeyCode::R => "r",
            KeyCode::S => "s",
            KeyCode::T => "t",
            KeyCode::U => "u",
            KeyCode::V => "v",
            KeyCode::W => "w",
            KeyCode::X => "x",
            KeyCode::Y => "y",
            KeyCode::Z => "z",
            // Top Row (Functions, Ecsape, Misc.)
            KeyCode::F1 => "f1",
            KeyCode::F2 => "f2",
            KeyCode::F3 => "f3",
            KeyCode::F4 => "f4",
            KeyCode::F5 => "f5",
            KeyCode::F6 => "f6",
            KeyCode::F7 => "f7",
            KeyCode::F8 => "f8",
            KeyCode::F9 => "f9",
            KeyCode::F10 => "10f",
            KeyCode::F11 => "f11",
            KeyCode::F12 => "f12",
            KeyCode::F13 => "f13",
            KeyCode::F14 => "f14",
            KeyCode::F15 => "f15",
            KeyCode::F16 => "f16",
            KeyCode::F17 => "f17",
            KeyCode::F18 => "f18",
            KeyCode::F19 => "f19",
            KeyCode::F20 => "f20",
            KeyCode::F21 => "f21",
            KeyCode::F22 => "f22",
            KeyCode::F23 => "f23",
            KeyCode::F24 => "f24",
            KeyCode::Escape => "esc",
            KeyCode::Snapshot => "prtsc",
            KeyCode::Scroll => "scrlk",
            KeyCode::Pause => "pause",

            // Navigation
            KeyCode::Insert => "insert",
            KeyCode::Home => "home",
            KeyCode::Delete => "delete",
            KeyCode::End => "end",
            KeyCode::PageDown => "pagedown",
            KeyCode::PageUp => "pageup",
            KeyCode::Left => "left",
            KeyCode::Up => "right",
            KeyCode::Right => "up",
            KeyCode::Down => "down",

            // Numpad
            KeyCode::Numlock => "numlock",
            KeyCode::Numpad0 => "numpad0",
            KeyCode::Numpad1 => "numpad1",
            KeyCode::Numpad2 => "numpad2",
            KeyCode::Numpad3 => "numpad3",
            KeyCode::Numpad4 => "numpad4",
            KeyCode::Numpad5 => "numpad5",
            KeyCode::Numpad6 => "numpad6",
            KeyCode::Numpad7 => "numpad7",
            KeyCode::Numpad8 => "numpad8",
            KeyCode::Numpad9 => "numpad9",
            KeyCode::NumpadAdd => "numpadadd",
            KeyCode::NumpadDivide => "numpaddiv",
            KeyCode::NumpadDecimal => "numpaddecimal",
            KeyCode::NumpadComma => "numpadcomma",
            KeyCode::NumpadEnter => "numpadenter",
            KeyCode::NumpadEquals => "numpadequals",
            KeyCode::NumpadMultiply => "numpadmul",
            KeyCode::NumpadSubtract => "numpadsub",
            // Misc.
            KeyCode::Space => "space",
            KeyCode::Back => "backspace",
            KeyCode::Return => "return",
            KeyCode::Tab => "tab",
            KeyCode::Compose => "compose",
            // Modifiers
            KeyCode::LAlt => "lalt",
            KeyCode::LControl => "lctrl",
            KeyCode::LShift => "lshift",
            KeyCode::LWin => "lwin",
            KeyCode::RControl => "rctrl",
            KeyCode::RAlt => "ralt",
            KeyCode::RShift => "rshift",
            KeyCode::RWin => "rwin",
            // Symbols
            KeyCode::Caret => "caret",
            KeyCode::Backslash => "bslash",
            KeyCode::Equals => "eq",
            KeyCode::LBracket => "lbracket",
            KeyCode::RBracket => "rbracket",
            KeyCode::Minus => "minus",
            KeyCode::Period => "period",
            KeyCode::Semicolon => "semicolon",
            KeyCode::Slash => "slash",
            KeyCode::Comma => "comma",
            // Unused
            KeyCode::Plus => "none",
            KeyCode::Apostrophe => "none",
            KeyCode::Asterisk => "none",
            KeyCode::Grave => "none",
            KeyCode::Colon => "none",
            // Media Control
            KeyCode::Mute => "none",
            KeyCode::PlayPause => "none",
            KeyCode::NextTrack => "none",
            KeyCode::PrevTrack => "none",
            KeyCode::VolumeDown => "none",
            KeyCode::VolumeUp => "none",
            KeyCode::MediaSelect => "none",
            KeyCode::MediaStop => "none",
            // Power
            KeyCode::Power => "none",
            KeyCode::Sleep => "none",
            KeyCode::Stop => "none",
            KeyCode::Wake => "none",
            KeyCode::Sysrq => "none",
            // OS key commands
            KeyCode::Copy => "none",
            KeyCode::Paste => "none",
            KeyCode::Cut => "none",
            KeyCode::Underline => "none",
            KeyCode::AbntC1 => "none",
            KeyCode::AbntC2 => "none",
            // Misc
            KeyCode::Unlabeled => "none",
            KeyCode::WebBack => "none",
            KeyCode::WebFavorites => "none",
            KeyCode::WebForward => "none",
            KeyCode::WebHome => "none",
            KeyCode::WebRefresh => "none",
            KeyCode::WebSearch => "none",
            KeyCode::WebStop => "none",
            KeyCode::Yen => "none",
            KeyCode::NavigateForward => "none",
            KeyCode::NavigateBackward => "none",
            KeyCode::NoConvert => "none",
            KeyCode::OEM102 => "none",
            KeyCode::At => "none",
            KeyCode::Ax => "none",
            KeyCode::Capital => "none",
            KeyCode::Convert => "none",
            KeyCode::Kana => "none",
            KeyCode::Kanji => "none",
            KeyCode::Mail => "none",
            KeyCode::Calculator => "none",
            KeyCode::MyComputer => "none",
            KeyCode::Apps => "none",
        },
        KeycodeType::Gamepad(buttoncode) => match buttoncode {
            Button::South => "gamepad_south",
            Button::East => "gamepad_east",
            Button::North => "gamepad_north",
            Button::West => "gamepad_west",
            Button::C => "gamepad_c",
            Button::Z => "gamepad_z",
            Button::LeftTrigger => "gamepad_l1",
            Button::LeftTrigger2 => "gamepad_l2",
            Button::RightTrigger => "gamepad_r1",
            Button::RightTrigger2 => "gamepad_r2",
            Button::Select => "gamepad_select",
            Button::Start => "gamepad_start",
            Button::Mode => "gamepad_mode",
            Button::LeftThumb => "gamepad_l3",
            Button::RightThumb => "gamepad_r3",
            Button::DPadUp => "gamepad_dup",
            Button::DPadDown => "gamepad_ddown",
            Button::DPadLeft => "gamepad_dleft",
            Button::DPadRight => "gamepad_dright",
            Button::Unknown => "gamepad_unknown",
        },
        KeycodeType::Mouse(mousekeycode) => match mousekeycode {
            MouseButton::Left => "mouse1",
            MouseButton::Right => "mouse2",
            MouseButton::Middle => "mouse3",
            MouseButton::Other(_othermousekeycode) => match _othermousekeycode {
                0 => "none",
                _ => unimplemented!(
                    "Mouse keycode {} is currently unimplemented",
                    _othermousekeycode
                ),
            },
        },
    };

    if str_ptr == "none" {
        return Err("Key not implemented");
    }

    Ok(str_ptr)
}
