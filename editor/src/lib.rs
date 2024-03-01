#![allow(unused)]
mod debug;
mod debuge {
    use bevy_ecs::prelude::*;
    use engine::input::{KeyCode, KeycodeType};
    use engine::Action;
    use engine::Input;

    static DEBUG_ACTION_NAME: &str = "debug_mode";

    fn setup_debug(mut input: ResMut<Input>) {
        let key = KeycodeType::Keyboard(KeyCode::Grave);
        let keys = vec![key];
        let action = Action::new(&mut input, DEBUG_ACTION_NAME.to_owned(), keys);
    }

    fn check_for_debug(input: Res<Input>) {
        if let Some(action) = input.get_action(DEBUG_ACTION_NAME) {
            if action.action_status().is_just_pressed() {}
        }
    }
}
