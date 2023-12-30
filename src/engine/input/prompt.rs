use crate::engine::input::KeycodeType;

use super::key::keycode_converter::str_to_keycode;

pub(super) fn prompt_string(p: &str) -> String {
    let mut io_input = String::new();
    loop {
        println!("{}", p);
        match std::io::stdin().read_line(&mut io_input) {
            Ok(_) => return io_input,
            Err(err) => println!("Invalid response [{}]", err),
        };
        io_input.clear();
    }
}

pub(super) fn prompt_bool(p: &str) -> bool {
    let mut io_input = String::new();
    loop {
        let value = prompt_string((String::from(p) + "[Y/n] >").as_str());
        {
            if value == "y" {
                return true;
            } else if value == "n" {
                return false;
            } else {
                println!("Not a valid boolean input [Must be 'y' or 'n', case insensitive]")
            }
        }
        io_input.clear();
    }
}

pub(super) fn prompt_int(p: &str) -> i32 {
    let mut io_input = String::new();
    loop {
        match prompt_string(p).parse::<i32>() {
            Ok(value) => return value,
            Err(err) => println!("Not a valid interger [{}]", err),
        }
        io_input.clear();
    }
}

pub(super) fn prompt_keycode(p: &str) -> KeycodeType {
    loop {
        match str_to_keycode(&prompt_string(p)) {
            Some(value) => break value,
            None => todo!(),
        }
    }
}
