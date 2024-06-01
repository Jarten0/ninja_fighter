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

pub fn init_editor_resources(world: &mut World) {
    world.insert_resource(InputDebugger::default());

    let editor_interface = world.resource_scope(
        |world: &mut World, mut engine: Mut<engine::GgezInterface>| {
            EditorGUI::new(engine.get_context_mut(), world)
        },
    );
    world.insert_resource(editor_interface);
}

/// An initialization function for a standard [`ScheduleTag::DebugGUI`] schedule.
pub fn debug_gui_schedule() -> Schedule {
    let mut sched = Schedule::new(ScheduleTag::DebugGUI);
    // Configuration block
    sched
        .set_build_settings(DEBUG_SETTINGS.clone())
        .set_executor_kind(ExecutorKind::Simple)
        .add_systems(inspector::update_windows);

    // Systems block

    sched
}

/// An initialization function for a standard [`ScheduleTag::DebugInit`] schedule.
pub fn debug_init_schedule() -> Schedule {
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

pub fn debug_tick_schedule() -> Schedule {
    let mut sched = Schedule::new(ScheduleTag::DebugTick);
    // Configuration block
    sched
        .set_build_settings(DEBUG_SETTINGS.clone())
        .set_executor_kind(ExecutorKind::Simple)
        .add_systems((console::check_for_debug).chain());

    // Systems block

    sched
}

pub fn debug_frame_schedule() -> Schedule {
    let mut sched = Schedule::new(ScheduleTag::DebugFrame);
    // Configuration block
    sched
        .set_build_settings(DEBUG_SETTINGS.clone())
        .set_executor_kind(ExecutorKind::Simple);

    // Systems block
    sched.add_systems(
        (
            // input_debugger::draw_debug_information,
            inspector::game_view::draw_editor_views,
            inspector::draw_editor_gui,
        )
            .chain(),
    );

    sched
}

pub static DEBUG_SETTINGS: ScheduleBuildSettings = ScheduleBuildSettings {
    ambiguity_detection: LogLevel::Warn,
    hierarchy_detection: LogLevel::Warn,
    use_shortnames: false,
    report_sets: true,
    auto_insert_apply_deferred: true,
};
