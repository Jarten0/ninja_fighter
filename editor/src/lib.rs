pub mod console;
pub mod input_debugger;
pub mod inspector;

use bevy_ecs::prelude::*;
use bevy_ecs::schedule::{ExecutorKind, LogLevel, ScheduleBuildSettings};
use engine::schedule::ScheduleTag;
use engine::GgezInterface;
use engine::{EngineConfig, Input};
use input_debugger::InputDebugger;
use inspector::Inspector;

static DEBUG_ACTION_NAME: &str = "debugconsole";

pub static EDITOR_ENGINE_CONFIG: EngineConfig = EngineConfig {
    scene_paths: &[game::INITIAL_SCENE],
    world_init: init_editor_schedules,
    schedule_builder_functions: crate::wrap_schedules_with_debug,
    ticks_per_second: game::ENGINE_CONFIG.ticks_per_second,
};

// Add new resources here!
pub fn init_editor_schedules(world: &mut World) {
    game::init_components_and_resources(world);
    world.insert_resource(InputDebugger::default());
    world.insert_resource(Inspector::default());
}

pub(crate) fn debug_schedule() -> Schedule {
    let mut sched = Schedule::new(ScheduleTag::DebugTick);
    // Configuration block
    sched
        .set_build_settings(DEBUG_SETTINGS.clone())
        .set_executor_kind(ExecutorKind::Simple);

    // Systems block

    sched
}

pub(crate) fn debug_frame_schedule() -> Schedule {
    let mut sched = Schedule::new(ScheduleTag::DebugFrame);
    // Configuration block
    sched
        .set_build_settings(DEBUG_SETTINGS.clone())
        .set_executor_kind(ExecutorKind::Simple);

    // Systems block
    sched.add_systems((
        inspector::draw_inspector,
        input_debugger::draw_debug_information,
    ));

    sched
}

pub(crate) static DEBUG_SETTINGS: ScheduleBuildSettings = ScheduleBuildSettings {
    ambiguity_detection: LogLevel::Warn,
    hierarchy_detection: LogLevel::Warn,
    use_shortnames: false,
    report_sets: true,
    auto_insert_apply_deferred: true,
};

pub fn wrap_schedules_with_debug() -> Vec<fn() -> Schedule> {
    let mut vec = vec![debug_schedule, debug_frame_schedule];
    vec.append(&mut game::schedule_builders());
    vec
}
