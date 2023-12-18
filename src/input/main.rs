/// Welcome to the input CLI editor! Here you can create new keys and alter the input data from the command line!
/// Mostly useful early on when setting things up.
pub fn main() -> ! {
    println!("Booting into input editor..");
    println!("\nWelcome to the input editor!");
    loop {
        // println!(">");
        let mut ioinput = String::new();
        if let Err(_) = std::io::stdin().read_line(&mut ioinput) {
            println!("Could not figure out what you asked, try again. Use `help` to see possible commands.");
            continue;
        }

        ioinput = ioinput.to_lowercase();

        // ioinput.pop();

        match Some(ioinput.clone()) {
            Some(input) if input == String::from("help") => help(),
            Some(input) if input == String::from("save") => save(),
            Some(input) if input == String::from("load") => load(),
            Some(input) if input == String::from("add_key") => add_key(),
            Some(input) if input == String::from("edit_key") => edit_key(),
            Some(input) if input == String::from("remove_key") => remove_key(),
            Some(input) if input == String::from("add_action") => add_action(),
            Some(input) if input == String::from("edit_action") => edit_action(),
            Some(input) if input == String::from("remove_action") => remove_action(),
            Some(input) if input == String::from("exit") => std::process::exit(0),
            Some(input) if input == String::from("") => help(),
            // Some(input) if input == "" => (),
            None => todo!(),
            _ => panic!("Error with given input: ` {} `", ioinput.clone()),
        }
    }
}

fn help() {
    println!(
        "
Welcome to my humble abode

No help is currently available, you'll have to pour through the code yourself

Sorry :\
    "
    )
}

fn save() {
    todo!()
}

fn load() {
    todo!()
}

fn add_key() {
    todo!()
}

fn edit_key() {
    todo!()
}

fn remove_key() {
    todo!()
}

fn add_action() {
    todo!()
}

fn edit_action() {
    todo!()
}

fn remove_action() {
    todo!()
}
