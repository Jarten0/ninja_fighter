mod collider;
mod debug;
mod protag;
mod render;
mod transform;

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
