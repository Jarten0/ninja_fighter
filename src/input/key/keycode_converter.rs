use std::{collections::HashMap, sync::OnceLock};

use ggez::input::keyboard::KeyCode;

fn hashmap() -> &'static HashMap<&'static str, KeyCode> {
    static HASHMAP: OnceLock<HashMap<&str, KeyCode>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        let mut hash_map = HashMap::new();
        hash_map.insert("1", KeyCode::Key1);
        hash_map.insert("2", KeyCode::Key2);
        hash_map.insert("3", KeyCode::Key3);
        hash_map.insert("4", KeyCode::Key4);
        hash_map.insert("5", KeyCode::Key5);
        hash_map.insert("6", KeyCode::Key6);
        hash_map.insert("7", KeyCode::Key7);
        hash_map.insert("8", KeyCode::Key8);
        hash_map.insert("9", KeyCode::Key9);
        hash_map.insert("0", KeyCode::Key0);
        hash_map.insert("a", KeyCode::A);
        hash_map.insert("b", KeyCode::B);
        hash_map.insert("c", KeyCode::C);
        hash_map.insert("d", KeyCode::D);
        hash_map.insert("e", KeyCode::E);
        hash_map.insert("f", KeyCode::F);
        hash_map.insert("g", KeyCode::G);
        hash_map.insert("h", KeyCode::H);
        hash_map.insert("i", KeyCode::I);
        hash_map.insert("j", KeyCode::J);
        hash_map.insert("k", KeyCode::K);
        hash_map.insert("l", KeyCode::L);
        hash_map.insert("m", KeyCode::M);
        hash_map.insert("n", KeyCode::N);
        hash_map.insert("o", KeyCode::O);
        hash_map.insert("p", KeyCode::P);
        hash_map.insert("q", KeyCode::Q);
        hash_map.insert("r", KeyCode::R);
        hash_map.insert("s", KeyCode::S);
        hash_map.insert("t", KeyCode::T);
        hash_map.insert("u", KeyCode::U);
        hash_map.insert("v", KeyCode::V);
        hash_map.insert("w", KeyCode::W);
        hash_map.insert("x", KeyCode::X);
        hash_map.insert("y", KeyCode::Y);
        hash_map.insert("y", KeyCode::Z);
        hash_map.insert("esc", KeyCode::Escape);
        hash_map.insert("f1", KeyCode::F1);
        hash_map.insert("f2", KeyCode::F2);
        hash_map.insert("f3", KeyCode::F3);
        hash_map.insert("f4", KeyCode::F4);
        hash_map.insert("f5", KeyCode::F5);
        hash_map.insert("f6", KeyCode::F6);
        hash_map.insert("f7", KeyCode::F7);
        hash_map.insert("f8", KeyCode::F8);
        hash_map.insert("f9", KeyCode::F9);
        hash_map.insert("f10", KeyCode::F10);
        hash_map.insert("f11", KeyCode::F11);
        hash_map.insert("f12", KeyCode::F12);
        hash_map.insert("f13", KeyCode::F13);
        hash_map.insert("f14", KeyCode::F14);
        hash_map.insert("f15", KeyCode::F15);
        hash_map.insert("f16", KeyCode::F16);
        hash_map.insert("f17", KeyCode::F17);
        hash_map.insert("f18", KeyCode::F18);
        hash_map.insert("f19", KeyCode::F19);
        hash_map.insert("f20", KeyCode::F20);
        hash_map.insert("f21", KeyCode::F21);
        hash_map.insert("f22", KeyCode::F22);
        hash_map.insert("f23", KeyCode::F23);
        hash_map.insert("f24", KeyCode::F24);
        hash_map.insert("insert", KeyCode::Insert);
        hash_map.insert("home", KeyCode::Home);
        hash_map.insert("delete", KeyCode::Delete);
        hash_map.insert("end", KeyCode::End);
        hash_map.insert("pagedown", KeyCode::PageDown);
        hash_map.insert("pageup", KeyCode::PageUp);
        hash_map.insert("left", KeyCode::Left);
        hash_map.insert("right", KeyCode::Up);
        hash_map.insert("up", KeyCode::Right);
        hash_map.insert("down", KeyCode::Down);
        hash_map.insert("backspace", KeyCode::Back);
        hash_map.insert("return", KeyCode::Return);
        hash_map.insert("space", KeyCode::Space);
        hash_map.insert("compose", KeyCode::Compose);
        hash_map.insert("caret", KeyCode::Caret);
        hash_map.insert("numlock", KeyCode::Numlock);
        hash_map.insert("numpad0", KeyCode::Numpad0);
        hash_map.insert("numpad1", KeyCode::Numpad1);
        hash_map.insert("numpad2", KeyCode::Numpad2);
        hash_map.insert("numpad3", KeyCode::Numpad3);
        hash_map.insert("numpad4", KeyCode::Numpad4);
        hash_map.insert("numpad5", KeyCode::Numpad5);
        hash_map.insert("numpad6", KeyCode::Numpad6);
        hash_map.insert("numpad7", KeyCode::Numpad7);
        hash_map.insert("numpad8", KeyCode::Numpad8);
        hash_map.insert("numpad9", KeyCode::Numpad9);
        hash_map.insert("numpadadd", KeyCode::NumpadAdd);
        hash_map.insert("numpaddiv", KeyCode::NumpadDivide);
        hash_map.insert("numpaddecimal", KeyCode::NumpadDecimal);
        hash_map.insert("numpadcomma", KeyCode::NumpadComma);
        hash_map.insert("numpadenter", KeyCode::NumpadEnter);
        hash_map.insert("numpadequals", KeyCode::NumpadEquals);
        hash_map.insert("numpadmul", KeyCode::NumpadMultiply);
        hash_map.insert("numpadsub", KeyCode::NumpadSubtract);
        hash_map.insert("bslash", KeyCode::Backslash);
        hash_map.insert("eq", KeyCode::Equals);
        hash_map.insert("lalt", KeyCode::LAlt);
        hash_map.insert("lbracket", KeyCode::LBracket);
        hash_map.insert("lctrl", KeyCode::LControl);
        hash_map.insert("lshift", KeyCode::LShift);
        hash_map.insert("lwin", KeyCode::LWin);
        hash_map.insert("dash", KeyCode::Minus);
        hash_map.insert("period", KeyCode::Period);
        hash_map.insert("ralt", KeyCode::RAlt);
        hash_map.insert("rshift", KeyCode::RShift);
        hash_map.insert("rwin", KeyCode::RWin);
        hash_map.insert("semicolon", KeyCode::Semicolon);
        hash_map.insert("slash", KeyCode::Slash);
        hash_map.insert("tab", KeyCode::Tab);
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

pub fn str_to_keycode(str_ptr: &str) -> Option<KeyCode> {
    Some(hashmap().get(str_ptr)?.to_owned())
}

pub fn keycode_to_str(keycode: KeyCode) -> Option<&'static str> {
    let str_ptr = match keycode {
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
        KeyCode::Minus => "dash",
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
    };
    Some(str_ptr)
}
