#![allow(unused)]
pub mod debug;
pub mod debuge {
    use bevy_ecs::schedule::{ExecutorKind, LogLevel, ScheduleBuildSettings};
    use bevy_ecs::{prelude::*, world};
    use components::debug;
    use engine::input::{KeyCode, KeycodeType};
    use engine::schedule::{ScheduleTag, Scheduler};
    use engine::Input;
    use engine::{ActionData, GgezInterface};

    use crate::debug::debug_cli;

    static DEBUG_ACTION_NAME: &str = "debugmode";

    fn setup_debug(mut input: ResMut<Input>) {
        let key = KeycodeType::Keyboard(KeyCode::Grave);
        let keys = vec![key];
        let action = ActionData::new(&mut input, DEBUG_ACTION_NAME.to_owned(), keys);
    }

    fn check_for_debug(
        input: Res<Input>,
        mut engine: ResMut<GgezInterface>,
        mut commands: Commands,
    ) {
        if let Some(action) = input.get_action(DEBUG_ACTION_NAME) {
            if action.status().is_just_pressed() {
                engine.debug_mode = true;
            }
        }
    }

    pub(crate) fn debug_schedule() -> (Schedule, ScheduleTag) {
        let mut sched = Schedule::default();
        // Configuration block
        sched
            .set_build_settings(DEBUG_SETTINGS.clone())
            .set_executor_kind(ExecutorKind::Simple);

        // Systems block

        (sched, ScheduleTag::Debug)
    }

    pub(crate) static DEBUG_SETTINGS: ScheduleBuildSettings = ScheduleBuildSettings {
        ambiguity_detection: LogLevel::Warn,
        hierarchy_detection: LogLevel::Warn,
        use_shortnames: false,
        report_sets: true,
    };

    pub fn init_editor_schedules(world: &mut World) {
        game::init_components_and_resources(world);
        // world.init_component()
    }

    pub fn wrap_schedules_with_debug() -> Vec<fn() -> (Schedule, ScheduleTag)> {
        let tickf = || {
            let (mut tick_sched, tag) = game::tick_schedule();
            tick_sched
                .add_systems(components::debug::update)
                .add_systems(check_for_debug);

            (tick_sched, tag)
        };

        let drawf = || {
            let (mut draw_sched, tag) = game::frame_schedule();
            draw_sched.add_systems(components::debug::draw);
            (draw_sched, tag)
        };

        let initf = || {
            let (mut init_sched, tag) = game::init_schedule();
            // init_sched.add_systems(components::debug::init);
            (init_sched, tag)
        };

        log::trace!("Wrapped schedules with debug versions");
        println!("Wrapped schedules with debug versions");

        vec![tickf, drawf, initf, debug_schedule]
    }
}
