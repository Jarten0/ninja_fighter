use bevy_ecs::schedule::Schedule;

use crate::components::{Transform, TransformSettings};

use crate::{
    space::{Position, Velocity},
    Update,
};

pub fn schedule_systems(mut sched: Schedule) -> Schedule {
    sched
        .add_systems(<Transform as Update<(&mut Position, &Velocity, &TransformSettings)>>::update);

    sched
}
