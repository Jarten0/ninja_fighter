use bevy_ecs::schedule::Schedule;
use bevy_ecs::world::World;

pub fn init_world(world: &mut World) {
    let schedule = Schedule::new(engine::schedule::ScheduleTag::Tick);
    world.add_schedule(schedule);

    let schedule = Schedule::new(engine::schedule::ScheduleTag::Frame);
    world.add_schedule(schedule);

    components::init_components(world);

    editor::init_editor_resources(world);

    world.add_schedule(editor::debug_tick_schedule());
    world.add_schedule(editor::debug_frame_schedule());
    world.add_schedule(editor::debug_init_schedule());
    world.add_schedule(editor::debug_gui_schedule());
}
