use crate::engine::input::key::keycode_converter::str_to_keycode;

/// Uses the given [`InputModuleType`].
static INPUT_MODULE: InputModuleType = InputModuleType::EmptyInputModule;

/// If [`Some`], pass in this value as the action's name to parse. May cause test to fail, and if so, try for the error message you expect using [`EXPECT_STRING_PARSE_ERROR`].
/// If [`None`], pass in the default parsing value.
static STRING_TEST_NAME: Option<&'static str> = None;

/// If [`Some`], pass in this value as the action keys to parse. May cause test to fail, and if so, try for the error message you expect using [`EXPECT_STRING_PARSE_ERROR`].
/// If [`None`], pass in the default parsing value.
static STRING_TEST_KEYS: Option<&'static str> = None;

/// If [`Some`], expect the string parse to fail with [`Err(EXPECT_STRING_PARSE_ERROR)`].
/// If [`None`], expect the string parse to return [`Ok(Action)`].
static EXPECT_STRING_PARSE_ERROR: Option<&'static str> = None;

// Test code below
/// Configures how the test will be run using any of the listed variants.
#[derive(Clone)]
#[allow(dead_code)]
enum InputModuleType {
    /// Uses a fresh [`Input`] module with no previous [`Action`]s stored for the test.
    EmptyInputModule,
    /// Uses the given closure to instantiate an [`Input`] module for the test.
    MockInputModule(fn() -> super::Input),
    /// Converts the given [`String`] into an [`Input`] module, then uses it for the test.
    MockInputModuleString(&'static str),
    /// Loads the current stored [`Input`] module using `Input::load()`, then uses it for the test.
    /// Will not mutate the stored data.
    StoredInputModule,
}

/// Runs a test on several string parsing actions.
/// Includes `Action::to_string` and `Action::from_str`
///
/// Configure block is declared above the func. (^up there^ if you're reading this documentation directly)
///
/// # Defaults
///
/// * string_action value: `"Test Name/key1;key2;key3"`
///
/// * action name: `"Test Name"`
///
/// * action keys: `"key1;key2;key3"`
///
/// # Special Characters
///
/// `/`: name and keys seperator
///
/// `;`: key seperator
#[test]
fn convert_action_to_str_test() {
    use crate::engine::{input::input_resource::Input, Action};
    use std::str::FromStr;

    // Initialize data
    let mut input_module: Input = match INPUT_MODULE {
        InputModuleType::EmptyInputModule => Input::new(),
        InputModuleType::MockInputModule(input_fn) => input_fn(),
        InputModuleType::MockInputModuleString(s) => Input::from_str(s).unwrap(),
        InputModuleType::StoredInputModule => Input::load(),
    };

    let test_name = match STRING_TEST_NAME {
        Some(name) => name,
        None => "Test Name",
    };

    let test_keys_str = match STRING_TEST_KEYS {
        Some(keys) => keys,
        None => "1;2;3;",
    };

    let mut keys = Vec::new();
    let mut key_buf = String::new();

    for key_char in test_keys_str.chars() {
        if key_char == ';' {
            push_key_to_keylist(&mut keys, &key_buf);
            key_buf.clear();
        } else {
            key_buf.push(key_char);
        }
    }
    if key_buf.len() > 0 {
        push_key_to_keylist(&mut keys, &key_buf)
    }

    let _key = str_to_keycode(test_keys_str);

    let instantiated_action: Action = Action {
        name: String::from(test_name),
        keys,
        status: Default::default(),
        default_keys: Vec::new(),
    };

    let test_action = String::from("|") + test_name + "/" + test_keys_str;

    // Test Functions
    let string_action = Action::to_string(&instantiated_action.clone());

    println!("{}", string_action);
    let result = Action::from_str(string_action.as_str()).unwrap();

    assert_eq!(string_action, test_action);
    assert_eq!(result.name, test_name);

    input_module.new_action(result);

    let parsed_action = input_module.get_action(&test_name).unwrap();

    assert_eq!(parsed_action, &instantiated_action);
}

fn push_key_to_keylist(keys: &mut Vec<super::KeycodeType>, key_buf: &String) {
    keys.push(
        str_to_keycode(key_buf.as_str())
            .ok_or(format!(
                "Failed to parse key in STRING_TEST_KEYS [{}]",
                key_buf
            ))
            .unwrap(),
    );
}
