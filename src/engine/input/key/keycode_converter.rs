use std::{collections::HashMap, sync::OnceLock};

use ggez::event::{Button, MouseButton};
use ggez::input::keyboard::KeyCode;

#[derive(Clone, Copy)]
pub enum KeyTypes {
    Keyboard(KeyCode),
    Gamepad(Button),
    Mouse(MouseButton),
}

fn hashmap() -> &'static HashMap<&'static str, KeyTypes> {
    static HASHMAP: OnceLock<HashMap<&str, KeyTypes>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        let mut hash_map = HashMap::new();

        {
            hash_map.insert("1", KeyTypes::Keyboard(KeyCode::Key1));
            hash_map.insert("2", KeyTypes::Keyboard(KeyCode::Key2));
            hash_map.insert("3", KeyTypes::Keyboard(KeyCode::Key3));
            hash_map.insert("4", KeyTypes::Keyboard(KeyCode::Key4));
            hash_map.insert("5", KeyTypes::Keyboard(KeyCode::Key5));
            hash_map.insert("6", KeyTypes::Keyboard(KeyCode::Key6));
            hash_map.insert("7", KeyTypes::Keyboard(KeyCode::Key7));
            hash_map.insert("8", KeyTypes::Keyboard(KeyCode::Key8));
            hash_map.insert("9", KeyTypes::Keyboard(KeyCode::Key9));
            hash_map.insert("0", KeyTypes::Keyboard(KeyCode::Key0));
            hash_map.insert("a", KeyTypes::Keyboard(KeyCode::A));
            hash_map.insert("b", KeyTypes::Keyboard(KeyCode::B));
            hash_map.insert("c", KeyTypes::Keyboard(KeyCode::C));
            hash_map.insert("d", KeyTypes::Keyboard(KeyCode::D));
            hash_map.insert("e", KeyTypes::Keyboard(KeyCode::E));
            hash_map.insert("f", KeyTypes::Keyboard(KeyCode::F));
            hash_map.insert("g", KeyTypes::Keyboard(KeyCode::G));
            hash_map.insert("h", KeyTypes::Keyboard(KeyCode::H));
            hash_map.insert("i", KeyTypes::Keyboard(KeyCode::I));
            hash_map.insert("j", KeyTypes::Keyboard(KeyCode::J));
            hash_map.insert("k", KeyTypes::Keyboard(KeyCode::K));
            hash_map.insert("l", KeyTypes::Keyboard(KeyCode::L));
            hash_map.insert("m", KeyTypes::Keyboard(KeyCode::M));
            hash_map.insert("n", KeyTypes::Keyboard(KeyCode::N));
            hash_map.insert("o", KeyTypes::Keyboard(KeyCode::O));
            hash_map.insert("p", KeyTypes::Keyboard(KeyCode::P));
            hash_map.insert("q", KeyTypes::Keyboard(KeyCode::Q));
            hash_map.insert("r", KeyTypes::Keyboard(KeyCode::R));
            hash_map.insert("s", KeyTypes::Keyboard(KeyCode::S));
            hash_map.insert("t", KeyTypes::Keyboard(KeyCode::T));
            hash_map.insert("u", KeyTypes::Keyboard(KeyCode::U));
            hash_map.insert("v", KeyTypes::Keyboard(KeyCode::V));
            hash_map.insert("w", KeyTypes::Keyboard(KeyCode::W));
            hash_map.insert("x", KeyTypes::Keyboard(KeyCode::X));
            hash_map.insert("y", KeyTypes::Keyboard(KeyCode::Y));
            hash_map.insert("y", KeyTypes::Keyboard(KeyCode::Z));
            hash_map.insert("esc", KeyTypes::Keyboard(KeyCode::Escape));
            hash_map.insert("f1", KeyTypes::Keyboard(KeyCode::F1));
            hash_map.insert("f2", KeyTypes::Keyboard(KeyCode::F2));
            hash_map.insert("f3", KeyTypes::Keyboard(KeyCode::F3));
            hash_map.insert("f4", KeyTypes::Keyboard(KeyCode::F4));
            hash_map.insert("f5", KeyTypes::Keyboard(KeyCode::F5));
            hash_map.insert("f6", KeyTypes::Keyboard(KeyCode::F6));
            hash_map.insert("f7", KeyTypes::Keyboard(KeyCode::F7));
            hash_map.insert("f8", KeyTypes::Keyboard(KeyCode::F8));
            hash_map.insert("f9", KeyTypes::Keyboard(KeyCode::F9));
            hash_map.insert("f10", KeyTypes::Keyboard(KeyCode::F10));
            hash_map.insert("f11", KeyTypes::Keyboard(KeyCode::F11));
            hash_map.insert("f12", KeyTypes::Keyboard(KeyCode::F12));
            hash_map.insert("f13", KeyTypes::Keyboard(KeyCode::F13));
            hash_map.insert("f14", KeyTypes::Keyboard(KeyCode::F14));
            hash_map.insert("f15", KeyTypes::Keyboard(KeyCode::F15));
            hash_map.insert("f16", KeyTypes::Keyboard(KeyCode::F16));
            hash_map.insert("f17", KeyTypes::Keyboard(KeyCode::F17));
            hash_map.insert("f18", KeyTypes::Keyboard(KeyCode::F18));
            hash_map.insert("f19", KeyTypes::Keyboard(KeyCode::F19));
            hash_map.insert("f20", KeyTypes::Keyboard(KeyCode::F20));
            hash_map.insert("f21", KeyTypes::Keyboard(KeyCode::F21));
            hash_map.insert("f22", KeyTypes::Keyboard(KeyCode::F22));
            hash_map.insert("f23", KeyTypes::Keyboard(KeyCode::F23));
            hash_map.insert("f24", KeyTypes::Keyboard(KeyCode::F24));
            hash_map.insert("insert", KeyTypes::Keyboard(KeyCode::Insert));
            hash_map.insert("home", KeyTypes::Keyboard(KeyCode::Home));
            hash_map.insert("delete", KeyTypes::Keyboard(KeyCode::Delete));
            hash_map.insert("end", KeyTypes::Keyboard(KeyCode::End));
            hash_map.insert("pagedown", KeyTypes::Keyboard(KeyCode::PageDown));
            hash_map.insert("pageup", KeyTypes::Keyboard(KeyCode::PageUp));
            hash_map.insert("left", KeyTypes::Keyboard(KeyCode::Left));
            hash_map.insert("right", KeyTypes::Keyboard(KeyCode::Up));
            hash_map.insert("up", KeyTypes::Keyboard(KeyCode::Right));
            hash_map.insert("down", KeyTypes::Keyboard(KeyCode::Down));
            hash_map.insert("backspace", KeyTypes::Keyboard(KeyCode::Back));
            hash_map.insert("return", KeyTypes::Keyboard(KeyCode::Return));
            hash_map.insert("space", KeyTypes::Keyboard(KeyCode::Space));
            hash_map.insert("compose", KeyTypes::Keyboard(KeyCode::Compose));
            hash_map.insert("caret", KeyTypes::Keyboard(KeyCode::Caret));
            hash_map.insert("numlock", KeyTypes::Keyboard(KeyCode::Numlock));
            hash_map.insert("numpad0", KeyTypes::Keyboard(KeyCode::Numpad0));
            hash_map.insert("numpad1", KeyTypes::Keyboard(KeyCode::Numpad1));
            hash_map.insert("numpad2", KeyTypes::Keyboard(KeyCode::Numpad2));
            hash_map.insert("numpad3", KeyTypes::Keyboard(KeyCode::Numpad3));
            hash_map.insert("numpad4", KeyTypes::Keyboard(KeyCode::Numpad4));
            hash_map.insert("numpad5", KeyTypes::Keyboard(KeyCode::Numpad5));
            hash_map.insert("numpad6", KeyTypes::Keyboard(KeyCode::Numpad6));
            hash_map.insert("numpad7", KeyTypes::Keyboard(KeyCode::Numpad7));
            hash_map.insert("numpad8", KeyTypes::Keyboard(KeyCode::Numpad8));
            hash_map.insert("numpad9", KeyTypes::Keyboard(KeyCode::Numpad9));
            hash_map.insert("numpadadd", KeyTypes::Keyboard(KeyCode::NumpadAdd));
            hash_map.insert("numpaddiv", KeyTypes::Keyboard(KeyCode::NumpadDivide));
            hash_map.insert("numpaddecimal", KeyTypes::Keyboard(KeyCode::NumpadDecimal));
            hash_map.insert("numpadcomma", KeyTypes::Keyboard(KeyCode::NumpadComma));
            hash_map.insert("numpadenter", KeyTypes::Keyboard(KeyCode::NumpadEnter));
            hash_map.insert("numpadequals", KeyTypes::Keyboard(KeyCode::NumpadEquals));
            hash_map.insert("numpadmul", KeyTypes::Keyboard(KeyCode::NumpadMultiply));
            hash_map.insert("numpadsub", KeyTypes::Keyboard(KeyCode::NumpadSubtract));
            hash_map.insert("bslash", KeyTypes::Keyboard(KeyCode::Backslash));
            hash_map.insert("eq", KeyTypes::Keyboard(KeyCode::Equals));
            hash_map.insert("lalt", KeyTypes::Keyboard(KeyCode::LAlt));
            hash_map.insert("lbracket", KeyTypes::Keyboard(KeyCode::LBracket));
            hash_map.insert("lctrl", KeyTypes::Keyboard(KeyCode::LControl));
            hash_map.insert("lshift", KeyTypes::Keyboard(KeyCode::LShift));
            hash_map.insert("lwin", KeyTypes::Keyboard(KeyCode::LWin));
            hash_map.insert("dash", KeyTypes::Keyboard(KeyCode::Minus));
            hash_map.insert("period", KeyTypes::Keyboard(KeyCode::Period));
            hash_map.insert("ralt", KeyTypes::Keyboard(KeyCode::RAlt));
            hash_map.insert("rshift", KeyTypes::Keyboard(KeyCode::RShift));
            hash_map.insert("rwin", KeyTypes::Keyboard(KeyCode::RWin));
            hash_map.insert("semicolon", KeyTypes::Keyboard(KeyCode::Semicolon));
            hash_map.insert("slash", KeyTypes::Keyboard(KeyCode::Slash));
            hash_map.insert("tab", KeyTypes::Keyboard(KeyCode::Tab));
        }
        // KeyCode::Snapshot => todo!(),
        // KeyCode::Scroll => todo!(),
        // KeyCode::Pause => todo!(),
        // KeyCode::AbntC1 => todo!(),
        // KeyCode::AbntC2 => todo!(),
        // KeyCode::Apostrophe => todo!(),
        // KeyCode::Apps => todo!(),
        // KeyCode::Asterisk => todo!(),
        // KeyCode::At => todo!(),
        // KeyCode::Ax => todo!(),
        // KeyCode::Calculator => todo!(),
        // KeyCode::Capital => todo!(),
        // KeyCode::Colon => todo!(),
        // KeyCode::Comma => todo!(),
        // KeyCode::Convert => todo!(),
        // KeyCode::Grave => todo!(),
        // KeyCode::Kana => todo!(),
        // KeyCode::Kanji => todo!(),
        // KeyCode::Mail => todo!(),
        // KeyCode::MediaSelect => todo!(),
        // KeyCode::MediaStop => todo!(),
        // KeyCode::Mute => todo!(),
        // KeyCode::MyComputer => todo!(),
        // KeyCode::NavigateForward => todo!(),
        // KeyCode::NavigateBackward => todo!(),
        // KeyCode::NextTrack => todo!(),
        // KeyCode::NoConvert => todo!(),
        // KeyCode::OEM102 => todo!(),
        // KeyCode::PlayPause => todo!(),
        // KeyCode::Plus => todo!(),
        // KeyCode::Power => todo!(),
        // KeyCode::PrevTrack => todo!(),
        // KeyCode::RBracket => todo!(),
        // KeyCode::RControl => todo!(),
        // KeyCode::Sleep => todo!(),
        // KeyCode::Stop => todo!(),
        // KeyCode::Sysrq => todo!(),
        // KeyCode::Underline => todo!(),
        // KeyCode::Unlabeled => todo!(),
        // KeyCode::VolumeDown => todo!(),
        // KeyCode::VolumeUp => todo!(),
        // KeyCode::Wake => todo!(),
        // KeyCode::WebBack => todo!(),
        // KeyCode::WebFavorites => todo!(),
        // KeyCode::WebForward => todo!(),
        // KeyCode::WebHome => todo!(),
        // KeyCode::WebRefresh => todo!(),
        // KeyCode::WebSearch => todo!(),
        // KeyCode::WebStop => todo!(),
        // KeyCode::Yen => todo!(),
        // KeyCode::Copy => todo!(),
        // KeyCode::Paste => todo!(),
        // KeyCode::Cut => todo!(),

        hash_map
    })
}

pub fn str_to_keycode(str_ptr: &str) -> Option<KeyTypes> {
    Some(hashmap().get(str_ptr)?.to_owned())
}

pub fn keycode_to_str(keytype: KeyTypes) -> Option<&'static str> {
    let str_ptr = match keytype {
        KeyTypes::Keyboard(keycode) => match keycode {
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
            KeyCode::Escape => "esc",
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
            KeyCode::Snapshot => todo!(),
            KeyCode::Scroll => todo!(),
            KeyCode::Pause => todo!(),
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
            KeyCode::Back => "backspace",
            KeyCode::Return => "return",
            KeyCode::Space => "space",
            KeyCode::Compose => "compose",
            KeyCode::Caret => "caret",
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
            KeyCode::AbntC1 => todo!(),
            KeyCode::AbntC2 => todo!(),
            KeyCode::Apostrophe => todo!(),
            KeyCode::Apps => todo!(),
            KeyCode::Asterisk => todo!(),
            KeyCode::At => todo!(),
            KeyCode::Ax => todo!(),
            KeyCode::Backslash => "bslash",
            KeyCode::Calculator => todo!(),
            KeyCode::Capital => todo!(),
            KeyCode::Colon => todo!(),
            KeyCode::Comma => todo!(),
            KeyCode::Convert => todo!(),
            KeyCode::Equals => "eq",
            KeyCode::Grave => todo!(),
            KeyCode::Kana => todo!(),
            KeyCode::Kanji => todo!(),
            KeyCode::LAlt => "lalt",
            KeyCode::LBracket => "lbracket",
            KeyCode::LControl => "lctrl",
            KeyCode::LShift => "lshift",
            KeyCode::LWin => "lwin",
            KeyCode::Mail => todo!(),
            KeyCode::MediaSelect => todo!(),
            KeyCode::MediaStop => todo!(),
            KeyCode::Minus => "minus",
            KeyCode::Mute => todo!(),
            KeyCode::MyComputer => todo!(),
            KeyCode::NavigateForward => todo!(),
            KeyCode::NavigateBackward => todo!(),
            KeyCode::NextTrack => todo!(),
            KeyCode::NoConvert => todo!(),
            KeyCode::OEM102 => todo!(),
            KeyCode::Period => "period",
            KeyCode::PlayPause => todo!(),
            KeyCode::Plus => todo!(),
            KeyCode::Power => todo!(),
            KeyCode::PrevTrack => todo!(),
            KeyCode::RAlt => "ralt",
            KeyCode::RBracket => todo!(),
            KeyCode::RControl => todo!(),
            KeyCode::RShift => "rshift",
            KeyCode::RWin => "rwin",
            KeyCode::Semicolon => "semicolon",
            KeyCode::Slash => "slash",
            KeyCode::Sleep => todo!(),
            KeyCode::Stop => todo!(),
            KeyCode::Sysrq => todo!(),
            KeyCode::Tab => "tab",
            KeyCode::Underline => todo!(),
            KeyCode::Unlabeled => todo!(),
            KeyCode::VolumeDown => todo!(),
            KeyCode::VolumeUp => todo!(),
            KeyCode::Wake => todo!(),
            KeyCode::WebBack => todo!(),
            KeyCode::WebFavorites => todo!(),
            KeyCode::WebForward => todo!(),
            KeyCode::WebHome => todo!(),
            KeyCode::WebRefresh => todo!(),
            KeyCode::WebSearch => todo!(),
            KeyCode::WebStop => todo!(),
            KeyCode::Yen => todo!(),
            KeyCode::Copy => todo!(),
            KeyCode::Paste => todo!(),
            KeyCode::Cut => todo!(),
        },
        KeyTypes::Gamepad(buttoncode) => match buttoncode {
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
        KeyTypes::Mouse(mousekeycode) => match mousekeycode {
            MouseButton::Left => "mouse1",
            MouseButton::Right => "mouse2",
            MouseButton::Middle => "mouse3",
            MouseButton::Other(_othermousekeycode) => unimplemented!(
                "Mouse keycode {} is currently unimplemented",
                _othermousekeycode
            ),
        },
    };

    todo!();
    // Some(str_ptr)
}
