//! The action module, which handles the clients interactive part of managing input data.
//!
//! Actions are containers for keys, and show whether any of the keys are active at the current moment.
//!
//! If any of the keys in an action are active, the action is considered active and will return as such.
//!
//! For more specific information, check out [`KeyStatus`] which handles whether a key has just been pressed,
//! and if the key has been held or released for a period of time.
// Woo! First file completley documented! (For now, anyway)

use super::key::keycode_converter::{keycode_to_str, str_to_keycode};
use super::Input;
use super::KeycodeType;
use crate::scene::SceneObjectID;
use crate::Key;
use std::{collections::HashMap, str::FromStr};

/// Declares the current state of the action.
///
/// If you want to simply know the binary `true`-`false` of whether a button is currently pressed, call the `is_held()` method to just read the value.
/// If you want a bit more detail though, like how long it's been held or if it has just been released, investigate the variants detailed below.
///
/// # Variants
///
/// * [`KeyStatus::Pressed`] - This action has been activated for one frame.
/// Will transition to `Held` after the frame is over.
///
/// * [`KeyStatus::Held(u32)`] - This action has been active for 2+ frames.
/// `u32` equals how many frames the button has been held, including the initial `Pressed` frame.
///
/// * [`KeyStatus::Released`] - This action has been deactivated for one frame.
/// Will transition to `Idle` after the frame is over.
///
///  * [`KeyStatus::Idle(u32)`] - This action has been deactivated for 2+ frames.
/// `u32` equals how many frames the button has been released, including the initial `Released` frame.
/// This is the default state of the action, so `u32` can equal one during initialization.
///
/// # Functions
///
/// * `is_held(&self) -> bool` - returns whether the action is currently active or not.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum KeyStatus {
    /// The key has just been pressed this frame
    Pressed,
    /// The key has been held for 2 or more frames
    Held(u32),
    /// The key has just been released this frame
    Released,
    /// The key has been released for 2 or more frames
    Idle(u32),
}

impl KeyStatus {
    /// Returns whether the action is currently active or not, regardless of if it's just been pressed or has been held.
    ///
    /// See also [`is_just_pressed`](KeyStatus::is_just_pressed) or [`is_just_released`](KeyStatus::is_just_released).
    pub fn is_held(&self) -> bool {
        match self {
            KeyStatus::Pressed => true,
            KeyStatus::Held(_) => true,
            KeyStatus::Released => false,
            KeyStatus::Idle(_) => false,
        }
    }

    /// Returns whether the action has just been pressed, but not if it's been held for more than one frame.
    /// Useful for triggering code once on an input.
    ///
    /// See also [`is_just_released`](KeyStatus::is_just_released)
    pub fn is_just_pressed(&self) -> bool {
        *self == KeyStatus::Pressed
    }

    /// Returns whether the action has just been pressed, but not if it's been released for more than one frame.
    /// Essentially just an inverse of [`is_just_pressed`](KeyStatus::is_just_pressed).
    pub fn is_just_released(&self) -> bool {
        *self == KeyStatus::Released
    }
}

impl Default for KeyStatus {
    /// Returns an [`KeyStatus::Idle`] variant that's been held for 0 frames.
    /// It's the only case where an `Idle` can have a value less than 2.
    fn default() -> Self {
        KeyStatus::Idle(0)
    }
}

impl core::fmt::Display for KeyStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyStatus::Pressed => write!(f, "Pressed"),
            KeyStatus::Held(frames) => write!(f, "Held({})", frames),
            KeyStatus::Released => write!(f, "Released"),
            KeyStatus::Idle(frames) => write!(f, "Idle({})", frames),
        }
    }
}

/// Container for an action. An action has a list of keys, and can be queried if any of them are currently active.
/// When any of the keys are pressed, `status` is set to [`KeyStatus::Pressed`].
///
/// It also contains a set of default keys that can only be changed outside of gameplay.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ActionData {
    pub name: String,
    pub id: ActionID,
    pub(crate) keys: Vec<KeycodeType>,
    pub(super) status: KeyStatus,
    pub(crate) default_keys: Vec<KeycodeType>,
}

impl ActionData {
    /// Creates a new action from the given keys and sets it into the [`Input`] resource.
    /// Returns a mutable reference to the action which is now owned by the [`Input`] module
    ///
    /// Use `Input::get_action` or `Input::get_action_mut` to alter the action later.
    pub fn new<'a>(
        input: &'a mut Input,
        name: String,
        keys: Vec<KeycodeType>,
    ) -> &'a mut ActionData {
        input.new_action(Self {
            name: name.clone(),
            keys,
            status: KeyStatus::Idle(0),
            default_keys: Vec::new(),
            id: ActionID::new(),
        });

        input.get_action_mut(&name).unwrap()
    }

    /// Adds a new [`Key`] to the key list for the [`Action`].
    ///
    /// Returns [`Ok`] if the key exists.
    ///
    /// Returns [`Err`] if the key does not exist.
    pub(crate) fn add_key(&mut self, key_str: KeycodeType) -> Result<(), &'static str> {
        self.keys.push(key_str);

        Ok(())
    }

    /// Removes a Key from the key list for the [`Action`]
    ///
    /// Returns [`Ok`] if the key if found
    ///
    /// Returns [`Err`] if the key is not found
    pub(crate) fn remove_key(&mut self, key: KeycodeType) -> Result<KeycodeType, &str> {
        match self.keys.binary_search(&key) {
            Ok(index) => Ok(self.keys.remove(index)),
            Err(_) => Err("Key doesn't exist, or the list is not sorted"),
        }
    }

    /// Updates the [`ActionData`]'s status based upon the keys in it's list.
    /// Prioritizes new key presses over key releases.
    pub(crate) fn update(&mut self, key_update_queue: &HashMap<KeycodeType, Key>) {
        let mut any_key_pressed_this_frame = false;
        let mut any_key_held_this_frame = false;
        for (keycode, key) in key_update_queue {
            if !self.keys.contains(keycode) {
                continue;
            }
            if key.status.is_held() {
                any_key_held_this_frame = true;
                if key.status.is_just_pressed() {
                    any_key_pressed_this_frame = true;
                    break;
                }
            }
        }
        if any_key_pressed_this_frame {
            self.status = KeyStatus::Pressed;
        } else if any_key_held_this_frame {
            match &mut self.status {
                KeyStatus::Pressed => self.status = KeyStatus::Held(2),
                KeyStatus::Held(i) => *i += 1,
                KeyStatus::Released => todo!(),
                KeyStatus::Idle(_) => todo!(),
            }
        } else if self.status.is_held() {
            self.status = KeyStatus::Released;
        } else {
            match self.status {
                KeyStatus::Released => self.status = KeyStatus::Idle(2),
                KeyStatus::Idle(i) => self.status = KeyStatus::Idle(i + 1),
                KeyStatus::Pressed | KeyStatus::Held(_) => self.status = KeyStatus::Released,
            }
        }
    }

    /// Gets the current status for whether the action is currently active or not.
    ///
    /// Check out [`KeyStatus`] for more details.
    pub fn status(&self) -> KeyStatus {
        self.status
    }

    /// The most basic check for input, is the button being pressed right now?
    /// Doesn't matter if it's been just pressed or how long, just says if it's currently active or not.
    ///
    /// For more specific checks, check out [`is_just_pressed`](ActionData::is_just_pressed),
    /// [`is_just_released`](ActionData::is_just_released) or [`status`](ActionData::status).
    /// Also check out [`KeyStatus`] for extra details.  
    pub fn is_pressed(&self) -> bool {
        self.status.is_held()
    }

    /// Checks if the button has just been pressed, and triggers only once when a button is pressed unlike [`is_pressed`](ActionData::is_pressed)
    /// which will keep returning `true` until the button is released.
    ///
    /// Also see [`is_just_released`](ActionData::is_just_released) for a release variant.
    pub fn is_just_pressed(&self) -> bool {
        self.status.is_just_pressed()
    }

    /// Checks if the button has just been released, similar to how [`is_just_pressed`](ActionData::is_just_pressed) works.
    pub fn is_just_released(&self) -> bool {
        self.status.is_just_released()
    }
}

impl ToString for ActionData {
    /// Return value example: `TestActionName/key1;key2;key3;|`
    fn to_string(&self) -> String {
        let mut output: String = String::new();

        output.push_str(&self.name);
        output.push('/');

        for key_type in &self.keys {
            output.push_str(keycode_to_str(key_type.clone()).unwrap());
            output.push_str(";")
        }
        output.push('|');

        // todo!();

        output
    }
}

impl FromStr for ActionData {
    /// sample value: "Action Name/key1;key2;key3|actionName2/key1;key4;key6|"
    ///
    /// `|`: action seperator
    ///
    /// `/`: name and keys seperator
    ///
    /// `;`: key seperator
    fn from_str(value: &str) -> Result<ActionData, &'static str> {
        let name_seperator = value.find('/').ok_or("action save value is invalid. (Could not find name and keys character seperator, aka '/')")?;

        let end_of_string = value.len();

        if let None = value.get(0..1) {
            return Err("action save value is invalid (Empty value given)");
        };

        let name = value
            .get(0..name_seperator)
            .ok_or("action save value is invalid. (Parsing the name of the action failed)")?
            .to_owned();

        let keys_str = value
            .get(name_seperator + 1..end_of_string)
            .ok_or("action save value is invalid. (keys_str is invalid)")?;

        let mut keys: Vec<KeycodeType> = Vec::new();

        let mut key_token_buf = String::new();
        for char in keys_str.chars() {
            if char == ';' {
                keys.push({
                    match str_to_keycode(key_token_buf.as_str()) {
                        Some(val) => val,
                        None => {
                            eprintln!("Invalid key type {}", key_token_buf.to_owned());
                            return Err("Invalid key type");
                        }
                    }
                });
                key_token_buf.clear();
            } else {
                key_token_buf.push(char)
            }
        }

        let action = ActionData {
            name,
            keys,
            status: KeyStatus::default(),
            default_keys: Vec::new(),
            id: ActionID::default(),
        };

        Ok(action)
    }

    type Err = &'static str;
}

/// A simple ID for uniquely identifying [`ActionData`]'s currently loaded.
///
/// Don't save ID's between different sessions, since the ID is assigned at runtime and does not take the action's factors into account
/// when assigning, instead just assigning on an incremental basis that can have differing results between reloads.
#[derive(Debug, Clone, Copy, Hash, PartialEq, PartialOrd, Eq)]
pub struct ActionID {
    id: usize,
}

impl Default for ActionID {
    /// Creates a new ID using an incremental static counter.
    ///
    /// Don't save ID's between sessions, since incremental counters don't keep track of actions after the program ends.
    fn default() -> Self {
        Self::new()
    }
}

impl ActionID {
    /// Creates a new ID using an incremental static counter.
    ///
    /// Don't save ID's between sessions, since incremental counters don't keep track of actions after the program ends.
    pub fn new() -> Self {
        Self {
            id: SceneObjectID::get_id_from_counter(crate::scene::CounterType::Actions),
        }
    }
}
