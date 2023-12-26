use std::{collections::HashMap, str::FromStr};

use super::{key::Key, resource};

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
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Action {
    pub name: String,
    pub(in crate::engine) keys: HashMap<&'static str, &'static Key>,
    pub(super) status: KeyStatus,
}

impl Action {
    /// Creates a new action from the given keys and sets it into the [`Input`] resource.
    /// Returns a mutable reference to the action which is now owned by the [`Input`] module
    ///
    /// Use `Input::get_action` or `Input::get_action_mut` to alter the action later.
    pub fn new<'a>(
        input: &'a mut resource::Input,
        name: String,
        keys: HashMap<&'static str, &'static Key>,
    ) -> &'a mut Action {
        input.new_action(Self {
            name: name.clone(),
            keys,
            status: KeyStatus::Idle(0),
        });

        return input.get_action_mut(&name).unwrap();
    }

    /// Adds a new [`Key`] to the key list for the [`Action`]
    pub fn add_key(&mut self, key: &'static Key) {
        self.keys.insert(key.name, key);
    }

    /// Removes a [`Key`] from the key list for the [`Action`]
    pub fn remove_key(&mut self, key: &'static Key) {
        self.keys.remove(key.name);
    }

    /// Updates the [`Action`]'s status based upon the keys in it's list. Prioritizes new key presses over key releases.
    pub fn update(&mut self) {
        let mut any_key_pressed_this_frame = false;
        let mut any_key_held_this_frame = false;

        for (_, key_value) in self.keys.iter() {
            match key_value.status {
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

        for (key_str, key) in &self.keys {
            output.push_str(key_str);
            output.push_str("_")
        }

        output.push('|');
        // todo!();

        output
    }
}

impl Action {
    /// sample value: "Action Name/key1;key2;key3|actionName2/key1;key4;key6|"
    ///
    /// `|`: action seperator
    ///
    /// `/`: name and keys seperator
    ///
    /// `;`: key seperator
    pub(in crate::engine) fn from_str(
        value: &str,
        input: &mut resource::Input,
    ) -> Result<Self, &'static str> {
        let name_seperator = value.find('/').expect(
            "action save value is invalid. (Could not find name and keys character seperator, aka '/')",
        );

        let end_of_string = value.len();

        let name = value
            .get(0..name_seperator)
            .expect("action save value is invalid. (Parsing the name of the action failed)")
            .to_owned();

        let keys_str = value
            .get(name_seperator..end_of_string)
            .expect("action save value is invalid. (keys_str is invalid)");

        let keys: HashMap<&str, &Key> = HashMap::new();

        let mut action = Action {
            name,
            keys,
            status: KeyStatus::default(),
        };

        let key_token_buf = String::new();

        for char in keys_str.chars() {
            if char == ';' {
                action.keys.insert(
                    &key_token_buf,
                    input.get_key_from_str(&key_token_buf).expect(
                        format!(
                            "action save value is invalid. (failed to parse key value {})",
                            key_token_buf
                        )
                        .as_str(),
                    ),
                );
            }
        }

        Ok(action)
    }
}
