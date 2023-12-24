use super::action::{Action, KeyStatus};
use super::key::Key;
use bevy_ecs::system::Resource;
use ggez::input::keyboard::KeyCode;
use std::collections::HashMap;
use std::env::current_dir;
use std::fs::File;
use std::io::prelude::*;

use std::path::PathBuf;
use std::str::FromStr;

/// A module for dealing with player input. Add it as a [`bevy_ecs::system::Res<>`] or [`bevy_ecs::system::ResMut<>`] parameter inside your system function to make use of it.
///
/// Unlike other engines, this input module makes use of Rust's rich type system to convey extra information without dealing with invalid state.
/// Once you have the resource, you can call `get_action(key)`, using `key` to try and find the action you want, ex. "Jump" or "Fire".
/// Then, you can store a reference to that action. From there, simply query any actions you want using `action_status()` to get a [`KeyStatus`] enum variant.
/// Read the [`KeyStatus`] documentation for further details on inquiring about actions.
#[derive(Resource)]
#[allow(dead_code)]
pub struct Input {
    actions: HashMap<String, Action>,
    keylist: HashMap<KeyCode, Key>,
    /// cli_mode is a special value, as it's equal to false during normal runtime. The only time it equates to true is when you are editing the Input module
    /// through the input CLI editor, which enables you to adjust keys specifically.
    cli_mode: bool,
}

#[allow(dead_code)]
impl Input {
    /// Returns a new [`Input`] resource. Should only be used when resetting the engine. Use `Input::load` unless you have a specific reason to use this.
    pub(crate) fn new() -> Self {
        Self {
            actions: HashMap::new(),
            keylist: HashMap::new(),
            cli_mode: false,
        }
    }

    /// Returns an [`Input`] resource with the keylist and whatever actions are currently stored in the engine save data. Recommended over `Input::new()`.
    pub(crate) fn load() -> Self {
        let mut input = Self::new();

        input.load_keys_file();

        input
    }

    pub fn action_status(&self, action: &Action) -> KeyStatus {
        action.status
    }

    /// Wrapper function for `HashMap::Insert` but making sure that the hash key equals the actions name.
    /// Thus, it also returns [`Some(Key)`] if the key already had a value. Otherwise, it returns [`None`].
    pub(super) fn new_action(&mut self, action: Action) -> Option<Action> {
        HashMap::insert(&mut self.actions, action.name.clone(), action)
    }

    /// Wrapper function for HashMap::get. Returns [`Some(&Action)`] if the action exists, and returning [`None`] if not.
    pub fn get_action(&mut self, action_name: &str) -> Option<&Action> {
        self.actions.get(action_name)
    }

    /// Wrapper function for HashMap::get_mut. Returns [`Some(&Action)`] if the action exists, and returning [`None`] if not.
    pub fn get_action_mut(&mut self, action_name: &str) -> Option<&mut Action> {
        self.actions.get_mut(action_name)
    }

    pub(in crate::engine) fn update_key(&mut self, key: &Key) {}
}

impl Input {
    pub(super) fn save_to_file(&self) {
        self.load_input_file(InputFile::ActionFile);
    }

    fn load_input_file(&self, filetype: InputFile) -> File {
        let dir = match current_dir() {
            Ok(path) => path,
            Err(err) => panic!("Path directory error! What? {}", err),
        };

        let file_path = dir.join(PathBuf::from(match filetype {
            InputFile::ActionFile => "assets\\engine\\input\\actionData.txt",
            InputFile::KeyFile => "assets\\engine\\input\\keyData.txt",
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
            .load_input_file(InputFile::KeyFile)
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
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut input = Self::new();
        let mut token_buf = String::new();

        for char in value.chars() {
            if char == ';' {
                let action = Action::from_str(&token_buf)?;
                input.new_action(action);
                token_buf = String::new();
            } else {
                token_buf.push(char);
            }
        }

        Ok(input)
    }
}

enum InputFile {
    KeyFile,
    ActionFile,
}
