//! Home to all game logic and custom built scripts.
//!
//! Any component files can be added and removed without worry.
//!
//! If you wish to make use of the engine, [`crate::engine`] is where you can access some of the public resources needed for interacting with the lower level logic.

mod collider;
mod debug;
mod protag;
mod render;
mod transform;

/// Takes in a [`bevy_ecs::system::Schedule`] and schedules all of the tick systems that operate
pub(crate) fn tick_schedule(sched: &mut bevy_ecs::schedule::Schedule) {
    sched.add_systems(transform::update);
    sched.add_systems(collider::collider_mesh::update);
    sched.add_systems(debug::update);
}

pub(crate) fn frame_schedule(draw_sched: &mut bevy_ecs::schedule::Schedule) {
    draw_sched.add_systems(render::draw);
    draw_sched.add_systems(debug::draw);
}

pub(crate) fn init_schedule(init_sched: &mut bevy_ecs::schedule::Schedule) {
    init_sched.add_systems(debug::init);
    init_sched.add_systems(protag::init);
}
