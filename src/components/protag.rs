use bevy_ecs::prelude::*;
use ggez::graphics::{self, DrawParam, GraphicsContext, Rect};

use crate::components::Transform;

use super::Renderer;

#[derive(Default, Component)]
pub struct Protag {}

#[derive(Bundle)]
pub struct ProtagBundle {
    pub protag: Protag,
    pub transform: Transform,
    pub renderer: Renderer,
}

impl ProtagBundle {
    pub fn default(gfx: &GraphicsContext) -> Self {
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

        Self {
            protag,
            transform,
            renderer,
        }
    }
}
