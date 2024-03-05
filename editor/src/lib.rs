#![allow(unused)]
mod debug;
pub mod debuge {
    use bevy_ecs::schedule::{ExecutorKind, LogLevel, ScheduleBuildSettings};
    use bevy_ecs::{prelude::*, world};
    use engine::input::{KeyCode, KeycodeType};
    use engine::schedule::{ScheduleTag, Scheduler};
    use engine::Action;
    use engine::Input;

    use crate::debug::debug_cli;

    static DEBUG_ACTION_NAME: &str = "debug_mode";

    fn setup_debug(mut input: ResMut<Input>) {
        let key = KeycodeType::Keyboard(KeyCode::Grave);
        let keys = vec![key];
        let action = Action::new(&mut input, DEBUG_ACTION_NAME.to_owned(), keys);
    }

    fn check_for_debug(input: Res<Input>, mut commands: Commands) {
        if let Some(action) = input.get_action(DEBUG_ACTION_NAME) {
            if action.action_status().is_just_pressed() {
                // debug_cli(root)
            }
        }
    }

    pub(crate) fn debug_schedule(sched: &mut Schedule) -> ScheduleTag {
        // Configuration block
        sched
            .set_build_settings(DEBUG_SETTINGS.clone())
            .set_executor_kind(ExecutorKind::Simple);

        // Systems block
        sched.add_systems(check_for_debug);

        ScheduleTag::Tick
    }

    pub(crate) static DEBUG_SETTINGS: ScheduleBuildSettings = ScheduleBuildSettings {
        ambiguity_detection: LogLevel::Warn,
        hierarchy_detection: LogLevel::Warn,
        use_shortnames: false,
        report_sets: true,
    };

    pub fn init_editor_schedules(world: &mut World) {
        game::game_data::init_components_and_resources(world);
        // world.init_component()
    }

    pub fn wrap_schedules_with_debug(
    ) -> Vec<for<'a> fn(&'a mut bevy_ecs::schedule::Schedule) -> ScheduleTag> {
        let mut builders = game::game_data::schedule_builders();
        builders.push(debug_schedule);
        builders
    }
}
