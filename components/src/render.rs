pub mod render_type;

use bevy_ecs::component::Component;
use bevy_ecs::prelude::*;
use bevy_ecs::system::Query;
use bevy_ecs::system::Res;
use bevy_ecs::system::ResMut;
use bevy_reflect::Reflect;
use engine::editor::FieldWidget;
use engine::Camera;
use ggez::graphics::{self as ggraphics, *};

use engine::space;
use engine::GgezInterface;
use ggraphics::Canvas;
use serde::Deserialize;
use serde::Serialize;

use self::render_type::RenderType;

use engine::space::Transform;

use engine::space::*;

type TransformComponentTuple<'a> = (
    &'a Position,
    &'a Velocity,
    &'a Rotation,
    &'a Scale,
    &'a TransformSettings,
);

pub fn update(mut query: Query<(&mut Renderer, TransformComponentTuple)>) {
    for (mut renderer, transform) in &mut query {
        let draw_transform: ggez::graphics::Transform = Transform {
            position: dbg!(transform.0.to_owned()),
            velocity: transform.1.to_owned(),
            rotation: transform.2.to_owned(),
            scale: transform.3.to_owned(),
            settings: transform.4.to_owned(),
        }
        .into();

        renderer.draw_param.transform = draw_transform.clone();

        let draw_param = renderer.draw_param.to_owned();

        Renderer::set(&mut renderer, draw_param, Vector2::new(0.0, 0.0))
    }
}

pub fn draw(
    query: Query<(&Renderer, TransformComponentTuple)>,
    mut main_canvas: ResMut<GgezInterface>,
    camera: Res<Camera>,
) {
    for (renderer, transform) in query.iter() {
        let canvas_option = main_canvas.get_canvas_mut();

        let canvas = match canvas_option {
            Some(canvas) => canvas,
            None => return,
        };

        if let Some(renderimage) = &renderer.image {
            match renderimage {
                RenderType::Image(image) => {
                    draw_image(canvas, image, renderer, transform, &camera);
                }
                RenderType::InstanceArray(_) => todo!(),
                RenderType::Mesh(_) => todo!(),
                RenderType::Text(_) => todo!(),
                RenderType::None => todo!(),
            }
        }
    }
}

fn draw_image(
    canvas: &mut Canvas,
    image: &Image,
    renderer: &Renderer,
    transform: TransformComponentTuple,
    camera: &Camera,
) {
    let mut transformer = DEFAULT_TRANSFORM.clone();

    transformer.position = {
        (*transform
            .0
            .to_owned()
            .translate(&renderer.offset)
            .translate(&-*camera.position))
        .into()
    };

    let mut draw_param = renderer.draw_param.clone();

    draw_param.transform = transformer.into();

    canvas.draw(image, draw_param)
}

#[derive(Component, Reflect, Default, Clone)]
#[reflect(Component)]
pub struct Renderer {
    #[reflect(ignore)]
    pub draw_param: DrawParam,
    #[reflect(ignore)]
    pub image: Option<RenderType>,
    pub offset: space::Vector2,
}

impl FieldWidget for Renderer {}

#[allow(dead_code)]
impl Renderer {
    /// Creates a new basic Renderer component for regular use.
    /// Use `Renderer::set()` to alter the offset and extra draw settings, or `Renderer::new_opt()` to directly set those values during initialization.
    pub fn new(image: Option<RenderType>, transform: Transform) -> Self {
        let draw_param = DrawParam {
            src: Default::default(),
            color: Color::WHITE,
            transform: transform.into(),
            z: 0,
        };

        let offset = space::Vector2::new(0.0, 0.0);

        Renderer {
            image,
            draw_param,
            offset,
        }
    }

    /// Similar to `Renderer::new()`, but with extra parameters for other values.
    pub fn new_opt(
        image: Option<RenderType>,
        draw_param: DrawParam,
        offset: space::Vector2,
    ) -> Self {
        Renderer {
            image,
            draw_param,
            offset,
        }
    }

    pub fn set(&mut self, draw_param: DrawParam, offset: space::Vector2) {
        self.draw_param = draw_param;
        self.offset = offset;
    }
}

#[allow(dead_code)]
impl Renderer {
    pub fn default() -> Self {
        Self {
            image: None,
            draw_param: Default::default(),
            offset: Default::default(),
        }
    }
}
