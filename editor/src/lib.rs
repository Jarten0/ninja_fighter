#![allow(unused)]

pub mod console;
pub mod input_debugger;
pub mod inspector;

use bevy_ecs::prelude::*;
use bevy_ecs::schedule::{ExecutorKind, LogLevel, ScheduleBuildSettings};
use engine::scene::SceneManager;
use engine::schedule::ScheduleTag;
use engine::GgezInterface;
use engine::{EngineConfig, Input};
use input_debugger::InputDebugger;
use log::trace;

use crate::inspector::EditorGUI;
use engine::editor::InspectableAsField;

static DEBUG_ACTION_NAME: &str = "enabledebugmode";

pub static EDITOR_ENGINE_CONFIG: EngineConfig = EngineConfig {
    scene_paths: &[game::INITIAL_SCENE],
    world_init: init_editor_schedules,
    ticks_per_second: game::ENGINE_CONFIG.ticks_per_second,
    freeze_on_unfocus: false,
    freeze_on_minimize: false,
    run_debug_schedules: true,
};

// Add new resources here!
pub fn init_editor_schedules(world: &mut World) {
    // The editor essentially acts as a wrapper for the game itself, injecting it's own code into the game.
    // Thus, we run all of the usual functions for the game, then we run our own stuff on top of it.
    game::init_components_and_resources(world);

    world.insert_resource(InputDebugger::default());

    let editor_interface =
        world.resource_scope(|world: &mut World, mut engine: Mut<GgezInterface>| {
            EditorGUI::new(engine.get_context_mut(), world)
        });
    world.insert_resource(editor_interface);

    world.add_schedule(debug_tick_schedule());
    world.add_schedule(debug_frame_schedule());
    world.add_schedule(debug_init_schedule());
    world.add_schedule(debug_gui_schedule());

    trace!("Created editor resources and initialized editor schedules");
}

pub(crate) fn debug_gui_schedule() -> Schedule {
    let mut sched = Schedule::new(ScheduleTag::DebugGUI);
    // Configuration block
    sched
        .set_build_settings(DEBUG_SETTINGS.clone())
        .set_executor_kind(ExecutorKind::Simple)
        .add_systems(inspector::update_windows);

    // Systems block

    sched
}

pub(crate) fn debug_init_schedule() -> Schedule {
    let mut sched = Schedule::new(ScheduleTag::DebugInit);
    // Configuration block
    sched
        .set_build_settings(DEBUG_SETTINGS.clone())
        .set_executor_kind(ExecutorKind::Simple)
        // .add_systems(())
        ;

    // Systems block

    sched
}

pub(crate) fn debug_tick_schedule() -> Schedule {
    let mut sched = Schedule::new(ScheduleTag::DebugTick);
    // Configuration block
    sched
        .set_build_settings(DEBUG_SETTINGS.clone())
        .set_executor_kind(ExecutorKind::Simple)
        .add_systems((console::check_for_debug).chain());

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
    sched.add_systems(
        (
            input_debugger::draw_debug_information,
            inspector::game_view::draw_game_view,
            inspector::draw_editor_gui,
        )
            .chain(),
    );

    sched
}

pub(crate) static DEBUG_SETTINGS: ScheduleBuildSettings = ScheduleBuildSettings {
    ambiguity_detection: LogLevel::Warn,
    hierarchy_detection: LogLevel::Warn,
    use_shortnames: false,
    report_sets: true,
    auto_insert_apply_deferred: true,
};
