use bevy_ecs::schedule::ExecutorKind;
use bevy_ecs::schedule::LogLevel;
use bevy_ecs::schedule::Schedule;
use bevy_ecs::schedule::ScheduleBuildSettings;

pub struct Scheduler {
    schedules: Vec<TaggedSchedule>,
}

impl Scheduler {
    pub fn new(e: Vec<fn(&mut Schedule) -> TaggedSchedule>) -> Self {
        let mut schedules = Vec::new();
        for i in e {
            // schedules.push(i())
        }

        Self { schedules }
    }
}

pub struct TaggedSchedule {
    schedule: Schedule,
    tag: ScheduleTag,
}

impl TaggedSchedule {
    fn new(schedule: Schedule, tag: ScheduleTag) -> Self {
        Self { schedule, tag }
    }
}

pub enum ScheduleTag {
    Tick,
    Frame,
    Init,
}
