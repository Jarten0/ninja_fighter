mod collider;
mod debug;
mod protag;
mod render;
mod transform;

/// Takes in a [`bevy_ecs::system::Schedule`] and schedules all of the tick systems that operate
pub(crate) fn tick_schedule(sched: &mut bevy_ecs::schedule::Schedule) {
    get_fns_from_components();
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

fn get_fns_from_components() {
    let files: std::fs::ReadDir;
    let mut path = std::env::current_dir().unwrap();
    path.push("src/components");

    if let Ok(ok) = std::fs::read_dir(path) {
        files = ok;
    } else {
        panic!("Unable to read files")
    };

    for file in files {
        if let Ok(ok) = file {
            let parse_file = syn::parse_file(ok.path().into_os_string().to_str().unwrap());
        }
    }
}

trait ComponentOrder {
    fn get_order(&self) -> i32;

    fn cmp(&self, other: &dyn ComponentOrder) {}
}
