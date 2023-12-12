use std::sync::Mutex;

use bevy_ecs::prelude::*;
use ggez::graphics::{self, DrawParam, Rect};

use crate::{components::Transform, GameInfo};

use super::{context::WorldInfo, Renderer};

#[derive(Default, Component)]
pub struct Protag {}

#[derive(Bundle)]
pub struct ProtagBundle {
    pub protag: Protag,
    pub transform: Transform,
    pub renderer: Renderer,
    pub world_info: WorldInfo,
}

impl ProtagBundle {
    pub fn default(game_info_ptr: *mut GameInfo) -> Self {
        let gfx = unsafe { &game_info_ptr.read().context_ptr.read().gfx };

        let transform = Transform {
            position: crate::space::Position::new(100.0, 100.0),
            velocity: crate::space::Velocity::default(),
            rotation: crate::space::Rotation::default(),
            scale: crate::space::Scale::default(),
            settings: super::TransformSettings::default(),
        };

        let mut renderer = Renderer::default(gfx);
        renderer.set(DrawParam {
            src: Rect::default(),
            color: graphics::Color::from_rgb(255, 0, 0),
            transform: transform.into(),
            z: 0,
        });

        let protag = Protag {};

        let world_info = WorldInfo {
            game_info: unsafe { game_info_ptr.read() },
        };

        Self {
            protag,
            transform,
            renderer,
            world_info,
        }
    }
}
