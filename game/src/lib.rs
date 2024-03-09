use bevy_ecs::schedule::*;
use components::*;
use engine::schedule::ScheduleTag;

pub fn init_components_and_resources(world: &mut bevy_ecs::world::World) {
    components::init_components(world);
}

pub fn schedule_builders() -> Vec<fn() -> (Schedule, ScheduleTag)> {
    vec![tick_schedule, frame_schedule, init_schedule]
}

pub fn tick_schedule() -> (Schedule, ScheduleTag) {
    let mut sched = Schedule::default();
    // Configuration block
    sched
        .set_build_settings(TICK_SETTINGS.clone())
        .set_executor_kind(ExecutorKind::MultiThreaded);

    // Systems block
    sched
        .add_systems(engine::systems::update)
        .add_systems(collider::collider_mesh::update)
        .add_systems(debug::update)
        .add_systems(collider::update)
        .add_systems(protag::update);

    (sched, ScheduleTag::Tick)
}

pub fn frame_schedule() -> (Schedule, ScheduleTag) {
    let mut draw_sched = Schedule::default();
    draw_sched
        .set_build_settings(FRAME_SETTINGS.clone())
        .set_executor_kind(ExecutorKind::SingleThreaded);

    draw_sched
        .add_systems(render::draw)
        .add_systems(debug::draw);

    (draw_sched, ScheduleTag::Frame)
}

pub fn init_schedule() -> (Schedule, ScheduleTag) {
    let mut init_sched = Schedule::default();
    init_sched
        .set_build_settings(INIT_SETTINGS.clone())
        .set_executor_kind(ExecutorKind::MultiThreaded);

    init_sched
        .add_systems(debug::init)
        .add_systems(protag::init);

    (init_sched, ScheduleTag::Init)
}

pub(crate) static TICK_SETTINGS: ScheduleBuildSettings = ScheduleBuildSettings {
    ambiguity_detection: LogLevel::Warn,
    hierarchy_detection: LogLevel::Warn,
    use_shortnames: false,
    report_sets: true,
};
pub(crate) static FRAME_SETTINGS: ScheduleBuildSettings = ScheduleBuildSettings {
    ambiguity_detection: LogLevel::Warn,
    hierarchy_detection: LogLevel::Warn,
    use_shortnames: false,
    report_sets: true,
};
pub(crate) static INIT_SETTINGS: ScheduleBuildSettings = ScheduleBuildSettings {
    ambiguity_detection: LogLevel::Warn,
    hierarchy_detection: LogLevel::Warn,
    use_shortnames: false,
    report_sets: true,
};
pub static TICKS_PER_SECOND: u32 = 60;
