use bevy_ecs::schedule::{self, Schedule, ScheduleBuildSettings};

use crate::components::{Renderer, Transform};

pub fn schedule_systems(mut sched: Schedule, mut draw_sched: Schedule) -> (Schedule, Schedule) {
    sched.set_build_settings(ScheduleBuildSettings {
        ambiguity_detection: schedule::LogLevel::Warn,
        hierarchy_detection: schedule::LogLevel::Warn,
        use_shortnames: false,
        report_sets: true,
    });
    draw_sched.set_build_settings(ScheduleBuildSettings {
        ambiguity_detection: schedule::LogLevel::Warn,
        hierarchy_detection: schedule::LogLevel::Warn,
        use_shortnames: false,
        report_sets: true,
    });

    sched.set_executor_kind(schedule::ExecutorKind::MultiThreaded);
    draw_sched.set_executor_kind(schedule::ExecutorKind::SingleThreaded);

    sched.add_systems(Transform::update);

    draw_sched.add_systems(Renderer::draw);
    (sched, draw_sched)
}
