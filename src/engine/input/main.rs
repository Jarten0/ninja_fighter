use crate::engine::input::key::keycode_converter::keycode_to_str;

use super::{Action, Input};

use inquire::{
    validator::{StringValidator, Validation},
    Text,
};

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

/// Welcome to the input CLI editor! Here you can create new keys and alter the input data from the command line!
/// Mostly useful early on when setting things up.
pub fn main() -> ! {
    println!("Booting into input editor..");
    println!("\nWelcome to the input editor!");
    println!("You can always hit enter with no input to exit the current prompt.");

    let mut input_module: Input = Input::new();
    loop {
        let char_limit = |input: &str| match input.chars().count() <= 20 {
            true => Ok(Validation::Valid),
            false => Ok(Validation::Invalid(
                "Max command length is 20 characters".into(),
            )),
        };

        let mut user_input = match Text::new("theeehee").with_validator(char_limit).prompt() {
            Ok(input) => input,
            Err(err) => {
                println!("Err? [{}]", err);
                continue;
            }
        };
        user_input = user_input.to_lowercase();

        if user_input == String::from("help") {
            help();
        } else if user_input == String::from("save") {
            save(&mut input_module);
        } else if user_input == String::from("load") {
            load(&mut input_module);
        } else if user_input == String::from("add_action") {
            add_action(&mut input_module);
        } else if user_input == String::from("edit_action") {
            edit_action(&mut input_module);
        } else if user_input == String::from("remove_action") {
            remove_action(&mut input_module);
        } else if user_input == String::from("exit") {
            let prompt =
                inquire::Confirm::new("Are you sure? Any unsaved work will be lost ").prompt();
            match prompt {
                Ok(..) => (),
                Err(_) => continue,
            }
            if prompt.unwrap() {
                std::process::exit(0);
            }
        } else {
            println!("Invalid input.");
        }
    }
}

fn help() {
    println!("{}", HOME_HELP_TEXT)
}

fn save(input: &mut Input) {
    input.save_to_file();
}

fn load(input: &mut Input) {
    *input = Input::load();

    // let dir = match std::env::current_dir() {
    //     Ok(path) => path,
    //     Err(err) => panic!("Path directory error! What? {}", err),
    // };

    // let key_path = dir.join(PathBuf::from("/src/input/keyData.txt"));
    // let mut key_file = match std::fs::File::open(key_path) {
    //     Ok(path) => path,
    //     Err(err) => panic!("Key file could not be opened! {}", err),
    // };

    // let mut buf = String::new();
    // match key_file.read_to_string(&mut buf) {
    //     Ok(_) => (),
    //     Err(err) => panic!("Invalid file read! {}", err),
    // }
    // let e = match Input::from_str(&buf) {
    //     Ok(e) => e,
    //     Err(e) => panic!("Uhoh {}", e),
    // };

    // *input = e;
}

fn add_action(input: &mut Input) {
    let name = prompt::prompt_string("Name of the action >");
    let mut keys = Vec::new();

    for key in 0..prompt::prompt_int("Number of keys >") {
        keys.push(prompt::prompt_keycode(
            format!("Name of key #{}", key).as_str(),
        ));
    }
    Action::new(input, name, keys);
}

fn edit_action(input_module: &mut Input) {
    println!("List of available actions: ");
    for (action_key, _) in &input_module.actions {
        print!("{}", action_key);
    }
    let action_ref = loop {
        match input_module.get_action_mut(&prompt_string("Which action do you want to edit?")) {
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
        let input_string = prompt_string("Which action do you want to take >").to_lowercase();
        let input_str = input_string.as_str();

        if input_str == "help" {
            println!("{}", ACTION_EDITOR_HELP_TEXT)
        } else if input_str == "add" {
            let _ = action_ref.add_key(prompt_keycode("Name of key to add"));
        } else if input_str == "clear" {
            action_ref.keys.clear();
        } else if input_str == "remove" {
            let _ = action_ref.remove_key(prompt_keycode("Name of key to remove"));
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
        let prompted_action =
            prompt_string("Which action would you like to remove? [Case sensitive]");

        let does_key_exist = input.does_key_exist(&prompted_action);

        if does_key_exist == false {
            println!("Action does not exist. [Are you sure the spelling is correct?]");
            continue;
        }

        if prompt_bool("Are you sure? ") {
            input.remove_action(&prompted_action);
        }
    }
}
