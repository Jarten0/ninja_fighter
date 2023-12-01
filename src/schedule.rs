use bevy_ecs::schedule::Schedule;


use crate::{transform::Transform, Update, space::{Position, Velocity}};

pub fn schedule_systems(mut sched: Schedule) -> Schedule {
    sched.add_systems(<Transform as Update<(&mut Position, &Velocity)>>::update);
    
    sched
}