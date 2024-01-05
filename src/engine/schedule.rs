use bevy_ecs::schedule::ExecutorKind;
use bevy_ecs::schedule::LogLevel;
use bevy_ecs::schedule::Schedule;
use bevy_ecs::schedule::ScheduleBuildSettings;

/// Builds the tick schedule and the frame schedule
///
/// The tick schedule runs all of the physics and game logic related systems
///
/// The frame schedule runs all of the drawing and rendering systems
///
/// The init schedule runs once for every init system
pub fn create_schedules() -> (Schedule, Schedule, Schedule) {
    let mut sched: Schedule = Schedule::default();
    sched.set_build_settings(ScheduleBuildSettings {
        ambiguity_detection: LogLevel::Warn,
        hierarchy_detection: LogLevel::Warn,
        use_shortnames: false,
        report_sets: true,
    });
    sched.set_executor_kind(ExecutorKind::MultiThreaded);
    crate::components::tick_schedule(&mut sched);

    let mut draw_sched: Schedule = Schedule::default();
    draw_sched.set_build_settings(ScheduleBuildSettings {
        ambiguity_detection: LogLevel::Warn,
        hierarchy_detection: LogLevel::Warn,
        use_shortnames: false,
        report_sets: true,
    });
    draw_sched.set_executor_kind(ExecutorKind::SingleThreaded);
    crate::components::frame_schedule(&mut draw_sched);

    let mut init_sched: Schedule = Schedule::default();
    init_sched.set_build_settings(ScheduleBuildSettings {
        ambiguity_detection: LogLevel::Warn,
        hierarchy_detection: LogLevel::Warn,
        use_shortnames: false,
        report_sets: true,
    });
    init_sched.set_executor_kind(ExecutorKind::MultiThreaded);
    crate::components::init_schedule(&mut init_sched);

    (sched, draw_sched, init_sched)
}
