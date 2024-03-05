use std::collections::HashMap;

use crate::input::key::keycode_converter::keycode_to_str;

use super::{Action, Input, KeycodeType};

use inquire::{validator::Validation, Confirm, CustomType, Text};

const HOME_HELP_TEXT: &str = "
Welcome to my humble abode

No help is currently available, you'll have to pour through the code yourself

Sorry :\
";

const ACTION_EDITOR_HELP_TEXT: &str = "
This is where you can edit the keybinds of the current action.

* `add` - adds a new key to the list of keys bound to this action.
Declare the name of the key to add. You can find a list at [todo]//TODO

* `remove` - removes a key from the list of keys bound to this action.
Declare the name of the key to remove. You can find a list at [todo]//TODO

* `clear` - empties the list of keys, making the action untriggerable until new keys are bound.

* `reset` - resets the list of keys using the default keybindings stored.
You can edit the default bindings using `edit`

* `edit` - TODO: allows you to change the current default keybinds of an action. //TODO

* `exit` - exits the key editor, returning to the main loop.
";

fn int_prompt_type() -> CustomType<'static, i32> {
    CustomType::new("Whaaa").with_default(0)
}

fn keycode_prompt_type() -> CustomType<'static, KeycodeType> {
    CustomType::new("Heehee").with_help_message("This is keycode")
}

/// Welcome to the input CLI editor! Here you can create new keys and alter the input data from the command line!
/// Mostly useful early on when setting things up.
pub fn main() -> ! {
    println!("Booting into input editor..");
    println!("\nWelcome to the input editor!");
    println!("You can always hit enter with no input to exit the current prompt.");

    let mut input_module: Input = Input::default();

    let char_limit = |input: &str| match input.chars().count() <= 20 {
        true => Ok(Validation::Valid),
        false => Ok(Validation::Invalid(
            "Max command length is 20 characters".into(),
        )),
    };

    let mut action_to_function: HashMap<&str, fn(&mut Input) -> ()> = HashMap::new();
    let vec: Vec<(&str, fn(&mut Input))> = vec![
        ("help", help),
        ("save", save),
        ("load", load),
        ("add_action", add_action),
        ("edit_action", edit_action),
        ("remove_action", remove_action),
    ];

    for (input, func) in vec {
        action_to_function.insert(input, func);
    }

    loop {
        let mut user_input = match Text::new("Pick a command")
            .with_validator(char_limit)
            .prompt()
        {
            Ok(input) => input,
            Err(err) => {
                println!("Err? [{}]", err);
                continue;
            }
        };
        user_input = user_input.to_lowercase();

        if user_input == String::from("exit") {
            let prompt = Confirm::new("Are you sure? Any unsaved work will be lost ").prompt();
            match prompt {
                Ok(..) => (),
                Err(_) => continue,
            }
            if prompt.unwrap() {
                std::process::exit(0);
            }
        } else if let Some(f) = action_to_function.get(user_input.as_str()) {
            f(&mut input_module)
        } else {
            println!("Invalid input.");
        }
    }
}

fn help(_: &mut Input) {
    println!("{}", HOME_HELP_TEXT)
}

fn save(input: &mut Input) {
    match Confirm::new("Are you sure? This will overwrite any previous saved data").prompt() {
        Ok(_response_yes) if _response_yes => input.save_to_file(),
        Ok(_response_no) => println!("Save aborted."),
        Err(err) => {
            eprintln!("Inquire error! Save aborted. [{}]", err);
            return;
        }
    }
}

fn load(input: &mut Input) {
    if input.actions.len() == 0 {
        *input = Input::load();
        return;
    }

    match Confirm::new(
        "Are you sure? This will overwrite the currently stored data, losing any unsaved work",
    )
    .prompt()
    {
        Ok(_response_yes) if _response_yes => *input = Input::load(),
        Ok(_response_no) => println!("Load aborted."),
        Err(err) => {
            eprintln!("Inquire error! Load aborted. [{}]", err);
            return;
        }
    }
}

fn add_action(input: &mut Input) {
    let name = Text::new("Name of the action >").prompt().unwrap();

    let mut keys = Vec::new();

    for _ in 0..int_prompt_type().prompt().unwrap() {
        keys.push(keycode_prompt_type().prompt().unwrap());
    }
    Action::new(input, name, keys);
}

fn edit_action(input_module: &mut Input) {
    println!("List of available actions: ");
    for (action_key, _) in &input_module.actions {
        print!("{}", action_key);
    }
    let action_ref = loop {
        match input_module.get_action_mut(
            &Text::new("Which action do you want to edit?")
                .prompt()
                .unwrap(),
        ) {
            Some(action) => break action,
            None => println!("Action does not exist"),
        }
    };

    let name = action_ref.name.clone();

    loop {
        println!("Edit the list of keys for {}", name);
        println!("List of keys: ");
        for key in action_ref.keys.clone() {
            println!("{}", keycode_to_str(key).unwrap());
        }
        println!("List of available actions: help, add, remove, clear, reset, rename, edit, exit");
        let input_string = Text::new("Which action do you want to take >")
            .prompt()
            .unwrap()
            .to_lowercase();
        let input_str = input_string.as_str();

        if input_str == "help" {
            println!("{}", ACTION_EDITOR_HELP_TEXT)
        } else if input_str == "add" {
            let _ = action_ref.add_key(
                keycode_prompt_type()
                    .with_help_message("Name of key to add")
                    .prompt()
                    .unwrap(),
            );
        } else if input_str == "clear" {
            action_ref.keys.clear();
        } else if input_str == "remove" {
            let _ = action_ref.remove_key(
                keycode_prompt_type()
                    .with_help_message("Name of key to remove")
                    .prompt()
                    .unwrap(),
            );
        } else if input_str == "exit" {
            break;
        }
    }
}

fn remove_action(input: &mut Input) {
    println!("List of available actions: ");
    for (action_key, _) in &input.actions {
        print!("{}", action_key);
    }
    loop {
        let prompted_action = Text::new("Which action would you like to remove? [Case sensitive]")
            .prompt()
            .unwrap();

        let does_key_exist = input.does_key_exist(&prompted_action);

        if does_key_exist == false {
            println!("Action does not exist. [Are you sure the spelling is correct?]");
            continue;
        }

        if Confirm::new("Are you sure? ").prompt().unwrap() {
            input.remove_action(&prompted_action);
        }
    }
}
