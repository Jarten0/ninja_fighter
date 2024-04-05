use bevy_ecs::schedule::*;
use components::*;
use engine::{schedule::ScheduleTag, EngineConfig};

pub static INITIAL_SCENE: &str = "game/assets/scenes/test_scene.json";

pub static ENGINE_CONFIG: EngineConfig = EngineConfig {
    scene_paths: &[INITIAL_SCENE],
    world_init: crate::init_components_and_resources,
    schedule_builder_functions: crate::schedule_builders,
    ticks_per_second: 60,
};

pub fn init_components_and_resources(world: &mut bevy_ecs::world::World) {
    components::init_components(world);
}

pub fn schedule_builders() -> Vec<fn() -> Schedule> {
    vec![tick_schedule, frame_schedule, init_schedule]
}

pub fn tick_schedule() -> Schedule {
    let mut sched = Schedule::new(ScheduleTag::Tick);
    // Configuration block
    sched
        .set_build_settings(TICK_SETTINGS.clone())
        .set_executor_kind(ExecutorKind::MultiThreaded)
        .add_systems((
            collider::update,
            protag::update,
            components::collider::mesh_editor::update_editor,
        ));

    sched
}

pub fn frame_schedule() -> Schedule {
    let mut draw_sched = Schedule::new(ScheduleTag::Frame);
    draw_sched
        .set_build_settings(FRAME_SETTINGS.clone())
        .set_executor_kind(ExecutorKind::SingleThreaded);

    draw_sched.add_systems((
        // insert draw systems here
        render::draw,
        components::collider::mesh_renderer::draw,
        components::collider::mesh_editor::draw_editor_interface,
    ));

    draw_sched
}

pub fn init_schedule() -> Schedule {
    let mut init_sched = Schedule::new(ScheduleTag::Init);

    init_sched
        .set_build_settings(INIT_SETTINGS.clone())
        .set_executor_kind(ExecutorKind::MultiThreaded);

    init_sched
        // .add_systems(debug::init)
        .add_systems(protag::init);

    init_sched
}

pub(crate) static TICK_SETTINGS: ScheduleBuildSettings = ScheduleBuildSettings {
    ambiguity_detection: LogLevel::Warn,
    hierarchy_detection: LogLevel::Warn,
    use_shortnames: false,
    report_sets: true,
    auto_insert_apply_deferred: true,
};
pub(crate) static FRAME_SETTINGS: ScheduleBuildSettings = ScheduleBuildSettings {
    ambiguity_detection: LogLevel::Warn,
    hierarchy_detection: LogLevel::Warn,
    use_shortnames: false,
    report_sets: true,
    auto_insert_apply_deferred: true,
};
pub(crate) static INIT_SETTINGS: ScheduleBuildSettings = ScheduleBuildSettings {
    ambiguity_detection: LogLevel::Warn,
    hierarchy_detection: LogLevel::Warn,
    use_shortnames: false,
    report_sets: true,
    auto_insert_apply_deferred: true,
};
