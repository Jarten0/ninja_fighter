use bevy_ecs::prelude::*;
use bevy_ecs::schedule::{Schedule, ScheduleBuildSettings, ScheduleLabel};
use egui::ahash::HashMap;

pub fn add_schedules(world: &mut World, schedule_builders: Vec<fn() -> Schedule>) {
    for builder in schedule_builders {
        world.add_schedule(builder())
    }
}

pub struct ScheduleBuilder {
    schedule_settings: ScheduleBuildSettings,
    systems: Vec<Box<dyn Any + IntoSystem<(), (), ()>>>,
    tag: ScheduleTag,
}

impl ScheduleBuilder {
    pub fn build(&self) -> Schedule {
        let mut schedule = Schedule::default();

        for system in self.systems {
            schedule.add_systems(system);
        }

        schedule
    }

    pub fn add_schedule(&mut self) {}
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
    DebugTick,
    DebugFrame,
    /// Is run every time a draw call will be made, regardless of whether the tick should be updating or not.
    /// Also runs regardless of freeze frames, since, well, I'd best hope you have access to your GUI when your game is deadlocked in a freeze loop.
    DebugGUI,
    DebugInit,
}
