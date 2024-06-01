use bevy_ecs::component::Component;
use bevy_ecs::reflect::ReflectComponent;
use bevy_ecs::system::Query;
use bevy_reflect::Reflect;
use engine::space::{self, Position, Velocity};
use serde::{Deserialize, Serialize};

/// A group of settings for controlling gravitational force for an entity.
///
#[derive(Debug, Component, Clone, Copy, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct GravitySettings {
    pub force: space::Vector2,
    /// The amount the field can be
    force_cap: f32,
    /// effectively force.normalized * force_cap, stored so that it doesn't have to be recalculated every frame.
    #[reflect(ignore)]
    terminal_force: space::Vector2,
}

impl GravitySettings {
    pub fn update_force_cap() {}
}

impl Default for GravitySettings {
    fn default() -> Self {
        Self {
            force: Default::default(),
            force_cap: Default::default(),
            terminal_force: Default::default(),
        }
    }
}

pub fn update_gravity_velocity(
    mut query: Query<(
        &GravitySettings,
        Option<&mut Velocity>,
        Option<&mut Position>,
    )>,
) {
    for (gravity, velocity, position) in query.iter_mut() {
        if let Some(mut velocity) = velocity {
            let gravity_vec = gravity.force.normalized();

            if velocity.x > gravity.force.x {
                let translation = gravity.force
                    - ((**velocity) - (gravity.force.normalized() * gravity.force_cap));

                velocity.translate(&translation);
            }

            velocity.translate(&gravity.force);
        } else {
            if let Some(mut position) = position {
                position.translate(&gravity.force);
            }
        }
    }
}
