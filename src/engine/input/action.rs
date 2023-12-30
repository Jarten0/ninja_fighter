use std::str::FromStr;

use super::{
    key::keycode_converter::{keycode_to_str, str_to_keycode},
    Input, KeycodeType,
};

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
pub(crate) enum KeyStatus {
    Pressed,
    Held(u32),
    Released,
    Idle(u32),
}

impl KeyStatus {
    pub fn is_held(&self) -> bool {
        match self {
            KeyStatus::Pressed => true,
            KeyStatus::Held(_) => true,
            KeyStatus::Released => false,
            KeyStatus::Idle(_) => false,
        }
    }
}

impl Default for KeyStatus {
    fn default() -> Self {
        KeyStatus::Idle(0)
    }
}

/// Container for an action. An action has a list of keys, and can be queried if any of them are currently active.
/// When any of the keys are pressed, `status` is set to [`KeyStatus::Pressed`].
///
/// It also contains a set of default keys that can only be changed outside of gameplay.
#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct Action {
    pub name: String,
    pub(in crate::engine) keys: Vec<KeycodeType>,
    pub(super) status: KeyStatus,
    pub(in crate::engine) default_keys: Vec<KeycodeType>,
}

impl Action {
    /// Creates a new action from the given keys and sets it into the [`Input`] resource.
    /// Returns a mutable reference to the action which is now owned by the [`Input`] module
    ///
    /// Use `Input::get_action` or `Input::get_action_mut` to alter the action later.
    pub fn new<'a>(input: &'a mut Input, name: String, keys: Vec<KeycodeType>) -> &'a mut Action {
        input.new_action(Self {
            name: name.clone(),
            keys,
            status: KeyStatus::Idle(0),
            default_keys: Vec::new(),
        });

        return input.get_action_mut(&name).unwrap();
    }

    /// Adds a new [`Key`] to the key list for the [`Action`].
    ///
    /// Returns [`Ok`] if the key exists.
    ///
    /// Returns [`Err`] if the key does not exist.
    pub fn add_key(&mut self, key_str: KeycodeType) -> Result<(), &'static str> {
        self.keys.push(key_str);

        Ok(())
    }

    /// Removes a Key from the key list for the [`Action`]
    ///
    /// Returns [`Ok`] if the key if found
    ///
    /// Returns [`Err`] if the key is not found
    pub fn remove_key(&mut self, key: KeycodeType) -> Result<KeycodeType, &str> {
        match self.keys.binary_search(&key) {
            Ok(index) => Ok(self.keys.remove(index)),
            Err(_) => Err("Key doesn't exist, or the list is not sorted"),
        }
    }

    /// Updates the [`Action`]'s status based upon the keys in it's list. Prioritizes new key presses over key releases.
    pub fn update(&mut self, input: &Input) {
        let mut any_key_pressed_this_frame = false;
        let mut any_key_held_this_frame = false;

        for reference_to_stored_key in self.keys.iter() {
            let key_with_current_status = match input.get_key(reference_to_stored_key) {
                Some(val) => val,
                None => panic!("called `Option::unwrap()` on a `None` value"),
            };

            match key_with_current_status.status {
                KeyStatus::Pressed => any_key_pressed_this_frame = true,
                KeyStatus::Held(_) => any_key_held_this_frame = true,
                KeyStatus::Released => (),
                KeyStatus::Idle(_) => (),
            };
        }

        if any_key_pressed_this_frame {
            // if any_key_pressed_this_frame
            self.status = KeyStatus::Pressed;
        } else {
            if any_key_held_this_frame {
                // if any_key_held_this_frame
                match &mut self.status {
                    KeyStatus::Pressed => self.status = KeyStatus::Held(2),
                    KeyStatus::Held(i) => *i += 1,
                    _ => unreachable!(),
                }
            } else {
                if self.status.is_held() {
                    // if self.status.is_held()
                    self.status = KeyStatus::Released;
                } else {
                    match self.status {
                        KeyStatus::Released => self.status = KeyStatus::Idle(2),
                        KeyStatus::Idle(i) => self.status = KeyStatus::Idle(i + 1),

                        KeyStatus::Pressed | KeyStatus::Held(_) => {
                            self.status = KeyStatus::Released
                        }
                    }
                }
            }
        }
    }
}

impl ToString for Action {
    fn to_string(&self) -> String {
        let mut output: String = String::new();

        output.push('|');
        output.push_str(&self.name);
        output.push('/');

        for key_type in &self.keys {
            output.push_str(keycode_to_str(key_type.clone()).unwrap());
            output.push_str(";")
        }

        // output.push('|');

        // todo!();

        output
    }
}

impl FromStr for Action {
    /// sample value: "Action Name/key1;key2;key3|actionName2/key1;key4;key6|"
    ///
    /// `|`: action seperator
    ///
    /// `/`: name and keys seperator
    ///
    /// `;`: key seperator
    fn from_str(value: &str) -> Result<Action, &'static str> {
        let name_seperator = value.find('/').ok_or("action save value is invalid. (Could not find name and keys character seperator, aka '/')")?;

        let end_of_string = value.len();

        match value.get(0..1) {
            Some(c) => {
                if !c.contains("|") {
                    println!("[{}]", c);
                    return Err(
                        "action save value is invalid (Invalid first character, must be '|')",
                    );
                }
            }
            None => return Err("action save value is invalid (Empty value given)"),
        };

        let name = value
            .get(1..name_seperator)
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
                            println!("Invalid key type {}", key_token_buf.to_owned());
                            return Err("Invalid key type");
                        }
                    }
                });
                key_token_buf.clear();
            } else {
                key_token_buf.push(char)
            }
        }

        let action = Action {
            name,
            keys,
            status: KeyStatus::default(),
            default_keys: Vec::new(),
        };

        Ok(action)
    }

    type Err = &'static str;
}
