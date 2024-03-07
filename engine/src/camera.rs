use crate::space;
use bevy_ecs::system::Resource;

#[allow(unused)]
#[derive(Resource)]
pub struct Camera {
    pub position: space::Position,
    pub velocity: space::Velocity,
    pub zoom: f32, // 1zm = 1cm/px or 1m/100px
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Default::default(),
            velocity: Default::default(),
            zoom: 1.0,
        }
    }
}
