use crate::engine::Key;

/// Uses the given [`InputModuleType`]
pub(crate) static INPUT_MODULE: InputModuleType = InputModuleType::EmptyInputModule;

/// If [`Some`], pass in this value as the action's name to parse. May cause test to fail, and if so, try for the error message you expect using [`EXPECT_STRING_PARSE_ERROR`]
///
/// If [`None`], pass in the default parsing value.
pub(crate) static STRING_TEST_NAME: Option<String> = None;

/// If [`Some`], pass in this value as the action keys to parse. May cause test to fail, and if so, try for the error message you expect using [`EXPECT_STRING_PARSE_ERROR`]
///
/// If [`None`], pass in the default parsing value.
pub(crate) static STRING_TEST_KEYS: Option<String> = None;

/// If [`Some`], expect the string parse to panic with [`Err(EXPECT_STRING_PARSE_ERROR)`].
///
/// If [`None`], expect the string parse to return [`Ok(Action)`]
pub(crate) static EXPECT_STRING_PARSE_ERROR: Option<String> = None;

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
pub(crate) fn convert_action_to_str_test() {
    use crate::engine::{input::resource::Input, Action};
    use std::collections::HashMap;
    use std::str::FromStr;

    let mut input_module = match INPUT_MODULE {
        InputModuleType::EmptyInputModule => Input::new(),
        InputModuleType::MockInputModule(input_fn) => input_fn(),
        InputModuleType::MockInputModuleString(s) => Input::from_str(s).unwrap(),
        InputModuleType::StoredInputModule => Input::load(),
    };

    let name = match STRING_TEST_NAME {
        Some(name) => name,
        None => String::from("Test Name"),
    };

    let mut instantiated_action = Action {
        name,
        keys: HashMap::new(),
        status: Default::default(),
    };

    let string_action = Action::to_string(&instantiated_action);

    assert_eq!(string_action, "TestName/key1;key2;key3");

    let parsed_action = Action::from_str(&string_action, &mut input_module).unwrap();

    assert_eq!(parsed_action.name, "TestName");
    assert_eq!(parsed_action, *instantiated_action);
}

/// Configures how the test will be run using any of the listed variants.
#[derive(Clone)]
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
