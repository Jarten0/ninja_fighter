use bevy_ecs::schedule::{self, Schedule, ScheduleBuildSettings, ScheduleLabel};

use crate::components::{Renderer, Transform, TransformSettings};

use crate::{
    space::{Position, Velocity},
    Update,
};

pub fn schedule_systems(mut sched: Schedule, mut draw_sched: Schedule) -> (Schedule, Schedule) {
    sched.set_build_settings(ScheduleBuildSettings {
        ambiguity_detection: schedule::LogLevel::Warn,
        hierarchy_detection: schedule::LogLevel::Warn,
        use_shortnames: false,
        report_sets: true,
    });

    sched.set_executor_kind(schedule::ExecutorKind::MultiThreaded);

    sched.add_systems(Transform::update);

    draw_sched.add_systems(Renderer::draw);
    (sched, draw_sched)
}
