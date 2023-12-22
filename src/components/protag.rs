use bevy_ecs::prelude::*;
use ggez::graphics::{self, Color, DrawParam, Image, Rect};

use crate::{components::Transform, engine::space, engine::MainCanvas};

use super::{Renderer, TransformSettings};

#[derive(Default, Component)]
pub struct Protag {}

#[derive(Bundle)]
pub struct ProtagBundle {
    pub protag: Protag,
    pub transform: Transform,
    pub renderer: Renderer,
}

impl ProtagBundle {
    pub fn default(game_info_ptr: &MainCanvas) -> Self {
        let protag = Protag {};

        let mut transform = Transform {
            position: space::Position::new(10.0, 10.0),
            velocity: space::Velocity::default(),
            rotation: space::Rotation::default(),
            scale: space::Scale::default(),
            settings: TransformSettings::default(),
        };

        transform.settings = super::TransformSettings {
            use_gravity: true,
            auto_update: true,
        };

        // transform.

        let gfx = &MainCanvas::get_context(&game_info_ptr).gfx;
        let mut renderer = Renderer::new(
            super::RenderType::Image(Image::from_color(gfx, 100, 100, Some(Color::RED))),
            transform.into(),
        );
        renderer.set(
            DrawParam {
                src: Rect::new_i32(10, 10, 1, 1),
                color: graphics::Color::WHITE,
                transform: transform.into(),
                z: 0,
            },
            space::Position::new(0.0, 0.0),
        );

        Self {
            protag,
            transform,
            renderer,
        }
    }
}
