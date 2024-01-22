use bevy_ecs::component::Component;

use crate::space;

#[derive(Component)]
struct Camera {
    position: space::Position,
    zoom: f32,
}
