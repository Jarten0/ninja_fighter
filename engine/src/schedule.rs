use std::collections::HashMap;

use bevy_ecs::schedule::ExecutorKind;
use bevy_ecs::schedule::LogLevel;
use bevy_ecs::schedule::Schedule;
use bevy_ecs::schedule::ScheduleBuildSettings;
use bevy_ecs::system::Resource;

#[derive(Resource)]
pub struct Scheduler {
    schedules: HashMap<ScheduleTag, Schedule>,
}

impl Scheduler {
    pub fn new(schedule_builders: Vec<fn(&mut Schedule) -> ScheduleTag>) -> Self {
        let mut schedules = HashMap::new();

        for builder in schedule_builders {
            let mut sched = Schedule::default();
            let tag = builder(&mut sched);

            schedules.insert(tag, sched);
        }

        Self { schedules }
    }

    pub fn get_schedule(&self, tag: ScheduleTag) -> Option<&Schedule> {
        self.schedules.get(&tag)
    }
    pub fn get_schedule_mut(&mut self, tag: ScheduleTag) -> Option<&mut Schedule> {
        self.schedules.get_mut(&tag)
    }
}

/// A value representing this schedule's behaviour, for when it should be run
#[derive(Hash, Eq, PartialEq)]
pub enum ScheduleTag {
    Tick,
    Frame,
    Init,
}
