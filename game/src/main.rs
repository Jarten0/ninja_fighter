//! The main binary.
//!
//! To create a game, first add the [`engine`] library as a dependency.
//! Then, add whichever component libraries you want to use.
//!
//! After that,
//!
//! * [`engine`] - modules designed to interface between the different libraries the core engine uses, including [`bevy_ecs`] and [`ggez`].
//! Also contains extra stuff to standardize things used between all projects that use this engine, for example, [`engine::space`] for a standard vector system.

/// The start of the program. The crux of the functionality however happens in [`engine::GameRoot`].
fn main() -> ! {
    let (mut context, event_loop) = ggez::ContextBuilder::new("Ninja Fighter", "Jarten0")
        .build()
        .expect("aieee, could not create ggez context!");

    // let scheduler = engine::schedule::Scheduler::new(create_schedules());
    let root = engine::GameRoot::new(&mut context);

    ggez::event::run(context, event_loop, root);
}

pub fn create_schedules() -> Vec<fn(&mut Schedule)> {
    todo!()
}
use bevy_ecs::schedule::ExecutorKind;
use bevy_ecs::schedule::LogLevel;
use bevy_ecs::schedule::Schedule;
use bevy_ecs::schedule::ScheduleBuildSettings;
use components::*;
pub fn tick_schedule() {
    let mut sched: Schedule = Schedule::default();
    sched.set_build_settings(ScheduleBuildSettings {
        ambiguity_detection: LogLevel::Warn,
        hierarchy_detection: LogLevel::Warn,
        use_shortnames: false,
        report_sets: true,
    });
    sched.set_executor_kind(ExecutorKind::MultiThreaded);

    sched.add_systems(transform::update);
    sched.add_systems(collider::collider_mesh::update);
    sched.add_systems(debug::update);
    sched.add_systems(collider::update);
}

// pub fn frame_schedule();
static DRAW_SCHED: Schedule = {
    let mut draw_sched: Schedule = Schedule::default();
    draw_sched.set_build_settings(ScheduleBuildSettings {
        ambiguity_detection: LogLevel::Warn,
        hierarchy_detection: LogLevel::Warn,
        use_shortnames: false,
        report_sets: true,
    });
    draw_sched.set_executor_kind(ExecutorKind::SingleThreaded);

    draw_sched.add_systems(render::draw);
    draw_sched.add_systems(debug::draw);
    draw_sched
};

static INIT_SETTINGS: ScheduleBuildSettings = ScheduleBuildSettings {
    ambiguity_detection: LogLevel::Warn,
    hierarchy_detection: LogLevel::Warn,
    use_shortnames: false,
    report_sets: true,
};
pub fn init_schedule() {
    let mut init_sched = Schedule::default();
    init_sched.set_build_settings(INIT_SETTINGS);
    init_sched.set_executor_kind(ExecutorKind::MultiThreaded);

    init_sched.add_systems(debug::init);
    init_sched.add_systems(protag::init);
}
