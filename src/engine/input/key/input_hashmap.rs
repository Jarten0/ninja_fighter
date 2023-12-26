use super::super::action::KeyStatus;
use super::super::key::keycode_converter;
use super::super::key::keycode_converter::KeyTypes;
use super::super::key::Key;
use super::KeyCode;
use crate::engine::input::key::stringcode::StringifiableKeyCode;
use std::collections::HashMap;
use std::sync::OnceLock;

pub(crate) enum InputFile {
    KeyFile,
    ActionFile,
}

pub(crate) fn const_key_hashmap() -> &'static HashMap<KeyTypes, Key> {
    static HASHMAP: OnceLock<HashMap<KeyTypes, Key>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        let mut hash_map = HashMap::new();
        {
            let keyvalue_pairs: Vec<KeyTypes> = vec![
                (KeyTypes::Keyboard(KeyCode::Key1)),
                (KeyTypes::Keyboard(KeyCode::Key2)),
                (KeyTypes::Keyboard(KeyCode::Key3)),
                (KeyTypes::Keyboard(KeyCode::Key4)),
                (KeyTypes::Keyboard(KeyCode::Key5)),
                (KeyTypes::Keyboard(KeyCode::Key6)),
                (KeyTypes::Keyboard(KeyCode::Key7)),
                (KeyTypes::Keyboard(KeyCode::Key8)),
                (KeyTypes::Keyboard(KeyCode::Key9)),
                (KeyTypes::Keyboard(KeyCode::Key0)),
                (KeyTypes::Keyboard(KeyCode::A)),
                (KeyTypes::Keyboard(KeyCode::B)),
                (KeyTypes::Keyboard(KeyCode::C)),
                (KeyTypes::Keyboard(KeyCode::D)),
                (KeyTypes::Keyboard(KeyCode::E)),
                (KeyTypes::Keyboard(KeyCode::F)),
                (KeyTypes::Keyboard(KeyCode::G)),
                (KeyTypes::Keyboard(KeyCode::H)),
                (KeyTypes::Keyboard(KeyCode::I)),
                (KeyTypes::Keyboard(KeyCode::J)),
                (KeyTypes::Keyboard(KeyCode::K)),
                (KeyTypes::Keyboard(KeyCode::L)),
                (KeyTypes::Keyboard(KeyCode::M)),
                (KeyTypes::Keyboard(KeyCode::N)),
                (KeyTypes::Keyboard(KeyCode::O)),
                (KeyTypes::Keyboard(KeyCode::P)),
                (KeyTypes::Keyboard(KeyCode::Q)),
                (KeyTypes::Keyboard(KeyCode::R)),
                (KeyTypes::Keyboard(KeyCode::S)),
                (KeyTypes::Keyboard(KeyCode::T)),
                (KeyTypes::Keyboard(KeyCode::U)),
                (KeyTypes::Keyboard(KeyCode::V)),
                (KeyTypes::Keyboard(KeyCode::W)),
                (KeyTypes::Keyboard(KeyCode::X)),
                (KeyTypes::Keyboard(KeyCode::Y)),
                (KeyTypes::Keyboard(KeyCode::Z)),
                (KeyTypes::Keyboard(KeyCode::Escape)),
                (KeyTypes::Keyboard(KeyCode::F1)),
                (KeyTypes::Keyboard(KeyCode::F2)),
                (KeyTypes::Keyboard(KeyCode::F3)),
                (KeyTypes::Keyboard(KeyCode::F4)),
                (KeyTypes::Keyboard(KeyCode::F5)),
                (KeyTypes::Keyboard(KeyCode::F6)),
                (KeyTypes::Keyboard(KeyCode::F7)),
                (KeyTypes::Keyboard(KeyCode::F8)),
                (KeyTypes::Keyboard(KeyCode::F9)),
                (KeyTypes::Keyboard(KeyCode::F10)),
                (KeyTypes::Keyboard(KeyCode::F11)),
                (KeyTypes::Keyboard(KeyCode::F12)),
                (KeyTypes::Keyboard(KeyCode::F13)),
                (KeyTypes::Keyboard(KeyCode::F14)),
                (KeyTypes::Keyboard(KeyCode::F15)),
                (KeyTypes::Keyboard(KeyCode::F16)),
                (KeyTypes::Keyboard(KeyCode::F17)),
                (KeyTypes::Keyboard(KeyCode::F18)),
                (KeyTypes::Keyboard(KeyCode::F19)),
                (KeyTypes::Keyboard(KeyCode::F20)),
                (KeyTypes::Keyboard(KeyCode::F21)),
                (KeyTypes::Keyboard(KeyCode::F22)),
                (KeyTypes::Keyboard(KeyCode::F23)),
                (KeyTypes::Keyboard(KeyCode::F24)),
                (KeyTypes::Keyboard(KeyCode::Insert)),
                (KeyTypes::Keyboard(KeyCode::Home)),
                (KeyTypes::Keyboard(KeyCode::Delete)),
                (KeyTypes::Keyboard(KeyCode::End)),
                (KeyTypes::Keyboard(KeyCode::PageDown)),
                (KeyTypes::Keyboard(KeyCode::PageUp)),
                (KeyTypes::Keyboard(KeyCode::Left)),
                (KeyTypes::Keyboard(KeyCode::Up)),
                (KeyTypes::Keyboard(KeyCode::Right)),
                (KeyTypes::Keyboard(KeyCode::Down)),
                (KeyTypes::Keyboard(KeyCode::Back)),
                (KeyTypes::Keyboard(KeyCode::Return)),
                (KeyTypes::Keyboard(KeyCode::Space)),
                (KeyTypes::Keyboard(KeyCode::Compose)),
                (KeyTypes::Keyboard(KeyCode::Caret)),
                (KeyTypes::Keyboard(KeyCode::Numlock)),
                (KeyTypes::Keyboard(KeyCode::Numpad0)),
                (KeyTypes::Keyboard(KeyCode::Numpad1)),
                (KeyTypes::Keyboard(KeyCode::Numpad2)),
                (KeyTypes::Keyboard(KeyCode::Numpad3)),
                (KeyTypes::Keyboard(KeyCode::Numpad4)),
                (KeyTypes::Keyboard(KeyCode::Numpad5)),
                (KeyTypes::Keyboard(KeyCode::Numpad6)),
                (KeyTypes::Keyboard(KeyCode::Numpad7)),
                (KeyTypes::Keyboard(KeyCode::Numpad8)),
                (KeyTypes::Keyboard(KeyCode::Numpad9)),
                (KeyTypes::Keyboard(KeyCode::NumpadAdd)),
                (KeyTypes::Keyboard(KeyCode::NumpadDivide)),
                (KeyTypes::Keyboard(KeyCode::NumpadDecimal)),
                (KeyTypes::Keyboard(KeyCode::NumpadComma)),
                (KeyTypes::Keyboard(KeyCode::NumpadEnter)),
                (KeyTypes::Keyboard(KeyCode::NumpadEquals)),
                (KeyTypes::Keyboard(KeyCode::NumpadMultiply)),
                (KeyTypes::Keyboard(KeyCode::NumpadSubtract)),
                (KeyTypes::Keyboard(KeyCode::Backslash)),
                (KeyTypes::Keyboard(KeyCode::Equals)),
                (KeyTypes::Keyboard(KeyCode::LAlt)),
                (KeyTypes::Keyboard(KeyCode::LBracket)),
                (KeyTypes::Keyboard(KeyCode::LControl)),
                (KeyTypes::Keyboard(KeyCode::LShift)),
                (KeyTypes::Keyboard(KeyCode::LWin)),
                (KeyTypes::Keyboard(KeyCode::Minus)),
                (KeyTypes::Keyboard(KeyCode::Period)),
                (KeyTypes::Keyboard(KeyCode::RAlt)),
                (KeyTypes::Keyboard(KeyCode::RShift)),
                (KeyTypes::Keyboard(KeyCode::RWin)),
                (KeyTypes::Keyboard(KeyCode::Semicolon)),
                (KeyTypes::Keyboard(KeyCode::Slash)),
                (KeyTypes::Keyboard(KeyCode::Tab)),
                // (KeyTypes::Gamepad(Button::)),
            ];

            for keytype in keyvalue_pairs {
                hash_map.insert(
                    keytype,
                    Key {
                        keycode: StringifiableKeyCode(keytype),
                        name: keycode_converter::keycode_to_str(keytype)
                            .expect("keytype is invalid"),
                        status: KeyStatus::default(),
                        event_occured: false,
                    },
                );
            }
        };

        hash_map
    })
}
