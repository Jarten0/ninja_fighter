use std::sync::Mutex;

use bevy_ecs::component::Component;

use crate::GameInfo;


#[derive(Component)]
pub struct GlobalInfo {
    pub game_info: Mutex<GameInfo>,
}