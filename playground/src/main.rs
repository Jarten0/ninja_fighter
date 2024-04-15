#![allow(unused)]

use bevy_ecs::schedule::Schedule;
use bevy_ecs::schedule::ScheduleLabel;
use bevy_ecs::world::World;

#[derive(Debug, Hash, PartialEq, Eq, Clone, ScheduleLabel)]
enum Label {
    Test,
}

pub struct TestingStaticBound<'a>
where
    Self: 'static,
{
    inner_struct: Inner<'a>,
}

pub struct Inner<'a>
where
    Self: 'static,
{
    non_static_lifetime: Option<&'a i32>,
}

fn main() {
    let mut test = TestingStaticBound {
        inner_struct: Inner {
            non_static_lifetime: None,
        },
    };
    {
        let i = 20;

        test.inner_struct.non_static_lifetime = Some(&i);

        println!("{}", test.inner_struct.non_static_lifetime.unwrap())
    }

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
