use crate::collider::BoxCollider;
use crate::{render::render_type::RenderType, render::Renderer};
use bevy_ecs::prelude::*;
use bevy_ecs::reflect::ReflectComponent;
use bevy_reflect::Reflect;

use engine::space::{Position, Transform, TransformSettings, Velocity};
use engine::GgezInterface;
use engine::{space, Input};
use ggez::graphics::{self, Color, DrawParam, Image, Rect};
use serde::{Deserialize, Serialize};

pub fn init(mut commands: Commands, _engine: Res<GgezInterface>) {
    // commands.spawn(ProtagBundle::new(&engine));
    commands
        .spawn(BoxCollider::new((100.0, 100.0).into()))
        .add(|mut entity: EntityWorldMut| {
            entity.insert(Position::new(0.0, 0.0));
        });
}

#[derive(Default, Component, Reflect, Clone, Debug)]
#[reflect(Component)]
pub struct Protag;

#[derive(Bundle)]
pub struct ProtagBundle {
    protag: Protag,
    controller: ProtagController,
    transform: Transform,
    renderer: Renderer,
    // collider: BoxCollider,
    // collider_mesh: ColliderMesh,
}

#[derive(Default, Debug, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
#[reflect(FromWorld)]
pub struct ProtagController {
    pub acc: f32,
    pub decel: f32,
    pub jump_power: f32,
    pub speed_cap: f32,
    pub fall_acc: f32,
    pub max_fall_speed: f32,
}

pub fn update(
    mut query: Query<(&mut Position, &mut Velocity, &ProtagController)>,
    input: Res<Input>,

    ggez: Res<GgezInterface>,
) {
    let screen_height = ggez.get_context().gfx.drawable_size().1;

    for (mut pos, mut velocity, controller) in query.iter_mut() {
        // if controller.speed_cap < 0

        if velocity.x.abs() > controller.speed_cap {
            velocity.x -= (controller.decel) * (velocity.x / velocity.x.abs());
        }

        if is_moving(WASD::D, &input) && velocity.x < controller.speed_cap {
            velocity.x += controller.acc;
        } else if is_moving(WASD::A, &input) && velocity.x > -controller.speed_cap {
            velocity.x -= controller.acc;
        } else {
            velocity.x *= 1.0 - controller.decel
        }
        if is_moving(WASD::W, &input) {
            velocity.y -= 0.3;
        }
        if is_moving(WASD::S, &input) {
            velocity.y += 0.3;
        }
        if (pos.y > 370.0 || true) && input.get_action("Click").unwrap().is_just_pressed() {
            velocity.y = -controller.jump_power;
            velocity.x *= 1.2;
            velocity.x = velocity
                .x
                .clamp(-controller.speed_cap * 1.2, controller.speed_cap * 1.2);
            //panics if speed_cap is negative
        }
        if velocity.y < controller.max_fall_speed && pos.y < screen_height - 110.0 {
            velocity.y += controller.fall_acc;
        } else if pos.y > screen_height - 100.0 && velocity.y >= 0.0 {
            velocity.y = 0.0;
            pos.y = screen_height - 100.0;
        }
    }
}

enum WASD {
    W,
    A,
    S,
    D,
}

fn is_moving(direction: WASD, input: &Input) -> bool {
    input
        .get_action(match direction {
            WASD::W => "Up",
            WASD::A => "Left",
            WASD::S => "Down",
            WASD::D => "Right",
        })
        .unwrap()
        .status()
        .is_held()
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
            Some(RenderType::Image(Image::from_color(
                gfx,
                100,
                100,
                Some(Color::RED),
            ))),
            transform.into(),
        );
        renderer.set(
            DrawParam {
                src: Rect::new_i32(10, 10, 1, 1),
                color: graphics::Color::WHITE,
                transform: transform.into(),
                z: 0,
            },
            space::Vector2::new(10.0, 0.0),
        );

        let mut bundle = BoxCollider::new(space::ONE);

        bundle.renderer.draw_param = Some(DrawParam {
            src: Rect::default(),
            color: Color::CYAN,
            transform: graphics::Transform::Values {
                dest: mint::Point2 { x: 20.0, y: 10.0 },
                rotation: 0.0,
                scale: space::ONE.into(),
                offset: mint::Point2 { x: 5.0, y: 0.0 },
            },
            z: 1,
        });

        let controller = ProtagController {
            acc: 0.5,
            decel: 0.08,
            jump_power: 10.0,
            speed_cap: 8.0,
            fall_acc: 0.5,
            max_fall_speed: 10.0,
        };
        Self {
            protag,
            transform,
            renderer,
            // collider,
            controller,
            // collider_mesh,
        }
    }
}
