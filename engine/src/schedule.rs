use std::any::Any;

use bevy_ecs::prelude::*;
use bevy_ecs::schedule::{Schedule, ScheduleBuildSettings, ScheduleLabel, SystemConfigs};
use egui::ahash::HashMap;

use crate::space::Position;

pub fn add_schedules(world: &mut World, schedule_builders: Vec<fn() -> Schedule>) {
    for builder in schedule_builders {
        world.add_schedule(builder())
    }
}

#[derive(Debug, Resource)]
pub struct ScheduleBuilder {
    schedule_settings: ScheduleBuildSettings,
    pub systems: Vec<fn()>,
    tag: ScheduleTag,
}

impl ScheduleBuilder {
    pub fn build(&self) -> Schedule {
        let mut schedule = Schedule::new(self.tag.clone());

        for system in &self.systems {
            schedule.add_systems(system.clone());
        }

        schedule.set_build_settings(self.schedule_settings.clone());

        schedule
    }

    pub fn add_schedule<M>(&mut self, system: fn()) {
        self.systems.push(system);
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
    DebugTick,
    DebugFrame,
    /// Is run every time a draw call will be made, regardless of whether the tick should be updating or not.
    /// Also runs regardless of freeze frames, since, well, I'd best hope you have access to your GUI when your game is deadlocked in a freeze loop.
    DebugGUI,
    DebugInit,
}
