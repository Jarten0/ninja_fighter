use bevy_ecs::system::Resource;

use crate::space;

#[derive(Resource)]
struct Camera {
    position: space::Position,
    zoom: f32,
}
