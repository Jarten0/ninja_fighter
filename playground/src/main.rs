use bevy_ecs::schedule::Schedule;
use bevy_ecs::schedule::ScheduleLabel;
use bevy_ecs::world::World;

#[derive(Debug, Hash, PartialEq, Eq, Clone, ScheduleLabel)]
enum Label {
    Test,
}

fn main() {
    let min = 5;
    let max = 0;
    println!("{}", !(max > min) || !(max < min))

    // let mut world = World::new();

    // let mut schedule = Schedule::new(Label::Test);

    // schedule.add_systems(test_system);

    // world.add_schedule(schedule);

    // for _ in 0..100 {
    //     println!("Calling run_schedule");
    //     world.run_schedule(Label::Test);
    // }
}

fn test_system() {
    println!("Ran schedule once")
}
