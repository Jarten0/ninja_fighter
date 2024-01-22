use bevy_ecs::schedule::ExecutorKind;
use bevy_ecs::schedule::LogLevel;
use bevy_ecs::schedule::Schedule;
use bevy_ecs::schedule::ScheduleBuildSettings;

pub struct Scheduler {
    schedules: Vec<(Schedule, ScheduleTag)>,
}

impl Scheduler {
    pub fn new(schedule_builders: Vec<fn(&mut Schedule) -> ScheduleTag>) -> Self {
        let mut schedules = Vec::new();

        for builder in schedule_builders {
            let mut sched = Schedule::default();
            let details = builder(&mut sched);

            schedules.push((sched, details))
        }

        Self { schedules }
    }
}

pub enum ScheduleTag {
    Tick,
    Frame,
    Init,
}
