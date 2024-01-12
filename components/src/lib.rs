#[allow(unused)]
mod collider;
mod debug;
mod protag;
mod render;
mod transform;

pub fn tick_schedule(sched: &mut bevy_ecs::schedule::Schedule) {
    sched.add_systems(transform::update);
    sched.add_systems(collider::collider_mesh::update);
    sched.add_systems(debug::update);
    sched.add_systems(collider::update);
}

pub fn frame_schedule(draw_sched: &mut bevy_ecs::schedule::Schedule) {
    draw_sched.add_systems(render::draw);
    draw_sched.add_systems(debug::draw);
}

pub fn init_schedule(init_sched: &mut bevy_ecs::schedule::Schedule) {
    init_sched.add_systems(debug::init);
    init_sched.add_systems(protag::init);
}
