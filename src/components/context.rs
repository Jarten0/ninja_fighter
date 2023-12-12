use std::sync::Mutex;

use bevy_ecs::component::Component;

use crate::GameInfo;

#[derive(Component)]
pub struct WorldInfo {
    pub game_info: GameInfo,
}
