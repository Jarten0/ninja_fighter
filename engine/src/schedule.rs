use bevy_ecs::prelude::*;
use bevy_ecs::schedule::{Schedule, ScheduleLabel};

pub fn add_schedules(world: &mut World, schedule_builders: Vec<fn() -> Schedule>) {
    for builder in schedule_builders {
        world.add_schedule(builder())
    }
}

/// A value representing this schedule's behaviour, for when it should be run
#[derive(Debug, ScheduleLabel, Hash, Eq, PartialEq, Clone, Copy)]
pub enum ScheduleTag {
    Tick,
    Frame,
    Init,
    DebugTick,
    DebugFrame,
    DebugInit,
}
