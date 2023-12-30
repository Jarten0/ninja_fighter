use super::action::{Action, KeyStatus};
use super::key::keycode_converter::KeycodeType;
use super::key::{input_hashmap, keycode_converter, Key};
use bevy_ecs::system::Resource;
use std::collections::{HashMap, LinkedList};
use std::env::current_dir;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;

/// A module for dealing with player input. Add it as a [`bevy_ecs::system::Res<>`] or [`bevy_ecs::system::ResMut<>`] parameter inside your system function to make use of it.
///
/// Unlike other engines, this input module makes use of Rust's rich type system to convey extra information without dealing with invalid state.
/// Once you have the resource, you can call `get_action(key)`, using `key` to try and find the action you want, ex. "Jump" or "Fire".
/// Then, you can store a reference to that action. From there, simply query any actions you want using `action_status()` to get a [`KeyStatus`] enum variant.
/// Read the [`KeyStatus`] documentation for further details on inquiring about actions.
///
/// # Key Update Queue
/// The process for updating keys goes along the lines of this:
///
/// * An event is triggered by the user
/// * [`ggez`] registers it and gives it to [`crate::engine::GameRoot`]
/// * [`crate::engine::GameRoot`] calls for an input assistant to store that information temporarily
/// * Once the next frame is ready to process, the input assistant gives the information to [`Input`] to deal with
/// * [`Input`] processes all of the updated information.
/// * Game logic can then use [`Input`]'s updated information for whatever it needs.
#[derive(Resource)]
#[allow(dead_code)]
pub(crate) struct Input
where
    Self: 'static,
{
    /// cli_mode is a special value, as it's equal to false during normal runtime. The only time it equates to true is when you are editing the Input module
    /// through the input CLI editor, which enables you to adjust keys specifically.
    ///
    ///  // Currently unused atm btw, don't know if its even useful. the docs lie since the code is still in development
    cli_mode: bool,
    // the rest of these are used in normal use cases
    pub(super) actions: HashMap<String, Action>,
    pub(super) keylist: HashMap<KeycodeType, Key>,
    pub(super) key_update_queue: LinkedList<(KeycodeType, bool)>,
}

#[allow(dead_code)]
impl Input {
    /// Returns a new [`Input`] resource. Should only be used when resetting the engine. Use `Input::load` unless you have a specific reason to use this.
    pub(in crate::engine) fn new() -> Self {
        Self {
            actions: HashMap::new(),
            keylist: HashMap::new(),
            cli_mode: false,
            key_update_queue: LinkedList::new(),
        }
    }

    /// Returns an [`Input`] resource with the keylist and whatever actions are currently stored in the engine save data. Recommended over `Input::new()`.
    pub(in crate::engine) fn load() -> Self {
        let mut input = Self::new();

        input.load_keys_file();

        input.keylist = (*input_hashmap::const_key_hashmap()).clone();

        input
    }

    pub(in crate::engine) fn update_key_queue(&mut self, key: KeycodeType, is_held: bool) {
        self.key_update_queue.push_front((key, is_held))
    }

    pub(in crate::engine) fn process_key_queue(&mut self) {
        for (_keycode, _key) in &mut self.keylist {}
    }

    pub(crate) fn does_key_exist(&self, key_str: &str) -> bool {
        let k = match keycode_converter::str_to_keycode(key_str) {
            Some(k) => k,
            None => return false,
        };
        self.keylist.contains_key(&k)
    }

    pub(in crate::engine) fn get_key(&self, key: &KeycodeType) -> Option<&Key> {
        self.keylist.get(key)
    }

    pub(in crate::engine) fn get_key_mut(&mut self, key: &KeycodeType) -> Option<&mut Key> {
        self.keylist.get_mut(key)
    }

    pub(in crate::engine) fn get_key_from_str(&self, key: &str) -> Option<&Key> {
        self.keylist.get(&keycode_converter::str_to_keycode(key)?)
    }

    /// Wrapper function for `HashMap::Insert` but making sure that the hash key equals the actions name.
    /// Thus, it also returns [`Some(Key)`] if the key already had a value. Otherwise, it returns [`None`].
    pub(in crate::engine) fn new_action(&mut self, action: Action) -> Option<Action> {
        HashMap::insert(&mut self.actions, action.name.clone(), action)
    }

    /// Returns the current status of the given action
    pub fn action_status(&self, action: &Action) -> KeyStatus {
        action.status
    }

    /// Wrapper function for `HashMap::get`. Returns [`Some(&Action)`] if the action exists, and returning [`None`] if not.
    pub fn get_action(&mut self, action_name: &str) -> Option<&Action> {
        self.actions.get(action_name)
    }

    /// Wrapper function for `HashMap::get_mut`.
    ///
    /// Returns [`Some`] if the action exists, and returning [`None`] if not.
    pub fn get_action_mut(&mut self, action_name: &str) -> Option<&mut Action> {
        self.actions.get_mut(action_name)
    }

    /// Wrapper function for `HashMap::remove`.
    ///
    /// Returns [`Some`] if the action exists.
    ///
    /// Returns [`None`] if the action doesn't exist.
    pub fn remove_action(&mut self, action_name: &str) -> Option<Action> {
        self.actions.remove(action_name)
    }
}

impl Input {
    pub(super) fn save_to_file(&self) {
        self.load_input_file(input_hashmap::InputFile::ActionFile);
    }

    fn load_input_file(&self, filetype: input_hashmap::InputFile) -> File {
        let dir = match current_dir() {
            Ok(path) => path,
            Err(err) => panic!("Path directory error! What? {}", err),
        };

        let file_path = dir.join(PathBuf::from(match filetype {
            input_hashmap::InputFile::ActionFile => "assets\\engine\\input\\actionData.txt",
            input_hashmap::InputFile::KeyFile => "assets\\engine\\input\\keyData.txt",
        }));

        match File::open(file_path.clone()) {
            Ok(path) => path,
            Err(err) => panic!(
                "Key file could not be opened! Error: [{}], Path: [{}]",
                err,
                file_path.display()
            ),
        }
    }

    fn load_keys_file(&mut self) {
        let mut key_buf = String::new();
        match self
            .load_input_file(input_hashmap::InputFile::KeyFile)
            .read_to_string(&mut key_buf)
        {
            Ok(_) => (),
            Err(err) => panic!("Invalid file read! {}", err),
        }

        let _ = Self::from_str(key_buf.as_str());
    }
}

impl ToString for Input {
    fn to_string(&self) -> String {
        let mut string = String::new();

        for (_, action) in self.actions.iter() {
            string.push_str(action.to_string().as_str());
            string.push_str("; ");
        }

        string
    }
}

impl FromStr for Input {
    type Err = &'static str;

    /// sample value: "Action Name/key1;key2;key3|actionName2/key1;key4;key6|"
    ///
    /// `|`: action seperator
    ///
    /// `/`: name and keys seperator
    ///
    /// `;`: key seperator
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let new_input_module = Self::new();
        let mut action_token_buf = String::new();

        for character in value.chars() {
            if character == '|' {
                Action::from_str(action_token_buf.as_str())?;

                action_token_buf = String::new();
            } else {
                action_token_buf.push(character);
            }
        }

        Ok(new_input_module)
    }
}
