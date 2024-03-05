use crate::{collider::Collider, render::render_type::RenderType, render::Renderer};

use engine::space::{Position, Transform, TransformSettings};

use engine::GgezInterface;
use engine::{space, Input};

use bevy_ecs::prelude::*;
use bevy_reflect::Reflect;
use ggez::graphics::{self, Color, DrawParam, Image, Rect};

pub fn init(mut commands: Commands, engine: Res<GgezInterface>) {
    commands.spawn(ProtagBundle::new(&engine));
}

#[derive(Default, Component, Reflect)]
pub struct Protag;

#[derive(Bundle)]
pub struct ProtagBundle {
    protag: Protag,
    controller: ProtagController,
    transform: Transform,
    renderer: Renderer,
    collider: Collider,
}

#[derive(Default, Debug, Component)]
pub struct ProtagController;

pub fn update(mut query: Query<&mut Position>, input: Res<Input>) {
    for mut position in query.iter_mut() {
        // dbg!(&input);
        if input
            .get_action("Right")
            .unwrap()
            .action_status()
            .is_just_pressed()
        {
            position.x += 5.0;
            println!("Shouldve moved")
        }
    }
}

impl ProtagBundle {
    pub fn new(engine: &GgezInterface) -> Self {
        let protag = Protag {};

        let transform = Transform {
            position: space::Position::new(10.0, 10.0),
            velocity: space::Velocity::default(),
            rotation: space::Rotation::default(),
            scale: space::Scale::default(),
            settings: TransformSettings { auto_update: true },
        };

        let gfx = &GgezInterface::get_context(&engine).gfx;
        let mut renderer = Renderer::new(
            RenderType::Image(Image::from_color(gfx, 100, 100, Some(Color::RED))),
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

        let collider = Collider::new(engine);

        Self {
            protag,
            transform,
            renderer,
            collider,
            controller: ProtagController,
        }
    }
}
