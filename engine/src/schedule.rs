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
    /// Runs game logic as normal.
    ///
    /// See also [`ScheduleTag::FreezeTick`] and [`crate::freeze::FreezeType`], or [`ScheduleTag::Frame`] for rendering
    Tick,
    /// Most game logic is paused at the moment, but some functionality can still run to maintain presentation.
    FreezeTick,
    /// The game is using the current stored state and rendering to the screen.
    Frame,
    /// The game has just been started, and is initializing state.
    /// This schedule is run only once, after resources and schedules have been created and inserted.
    Init,
    /// Runs during debug
    DebugTick,
    DebugFrame,
    DebugInit,
}
