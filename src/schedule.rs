use bevy_ecs::schedule::Schedule;


use crate::{Update, space::{Position, Velocity}};
use crate::components::Transform;

pub fn schedule_systems(mut sched: Schedule) -> Schedule {
    sched.add_systems(<Transform as Update<(&mut Position, &Velocity)>>::update);
    
    sched
}