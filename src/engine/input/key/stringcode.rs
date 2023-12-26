use std::str::FromStr;

use super::keycode_converter::{self, KeyTypes};
use ggez::input::keyboard::KeyCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct StringifiableKeyCode(pub KeyTypes);

impl std::ops::Deref for StringifiableKeyCode {
    type Target = KeyTypes;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ToString for StringifiableKeyCode {
    /// Returns a [`String`] value for [`StringifiableKeyCode`].
    /// Returns the string `"None"` if an unimplemented [`StringifiableKeyCode`] is given.
    ///
    /// Most normal keys are supported, but some keyboard model specific characters are exempt at the moment being.
    /// Everything standardly available on a modern keyboard is available though and a few more, including up to the `F1`-`F24` keys and the full numpad.
    /// Only exception to this are the media control keys due to their unrelated nature.
    fn to_string(&self) -> String {
        keycode_converter::keycode_to_str(self.0)
            .unwrap_or("None")
            .to_string()
    }
}

impl FromStr for StringifiableKeyCode {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match keycode_converter::str_to_keycode(s) {
            Some(str) => Ok(StringifiableKeyCode(str)),
            None => Err("Unavailable keycode"),
        }
    }
}
