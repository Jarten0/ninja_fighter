pub mod render_type;

use bevy_ecs::system::Query;
use bevy_ecs::system::ResMut;
use ggez::graphics::{self as ggraphics, *};

use crate::engine::space;
use crate::engine::Engine;

use super::transform::Transform;

#[derive(bevy_ecs::component::Component)]
pub struct Renderer {
    pub image: render_type::RenderType,
    pub draw_param: DrawParam,
    pub transform: super::transform::Transform,
    pub offset: space::Position,
}

#[allow(dead_code)]
impl Renderer {
    /// Creates a new basic Renderer component for regular use.
    /// Use `Renderer::set()` to alter the offset and extra draw settings, or `Renderer::new_opt()` to directly set those values during initialization.
    pub fn new(image: render_type::RenderType, transform: super::transform::Transform) -> Self {
        let draw_param = DrawParam {
            src: Default::default(),
            color: Color::WHITE,
            transform: transform.into(),
            z: 0,
        };
        let offset = space::Position::new(0.0, 0.0);

        Renderer {
            image,
            draw_param,
            transform,
            offset,
        }
    }

    /// Similar to `Renderer::new()`, but with extra parameters for other values.
    pub fn new_opt(
        image: render_type::RenderType,
        transform: Transform,
        draw_param: DrawParam,
        offset: space::Position,
    ) -> Self {
        Renderer {
            image,
            draw_param,
            transform,
            offset,
        }
    }

    pub fn set(&mut self, draw_param: DrawParam, offset: space::Position) {
        self.draw_param = draw_param;
        self.offset = offset;
    }
}

#[allow(dead_code)]
impl Renderer {
    pub fn default(gfx: &GraphicsContext) -> Self {
        Self {
            image: render_type::RenderType::default(gfx),
            draw_param: Default::default(),
            transform: Default::default(),
            offset: Default::default(),
        }
    }
}

pub fn draw(query: Query<&Renderer>, mut main_canvas: ResMut<Engine>) {
    for renderer in query.iter() {
        let canvas_option = main_canvas.get_canvas_mut();

        let mut canvas = match canvas_option {
            Some(canvas) => canvas,
            None => return,
        };

        match &renderer.image {
            render_type::RenderType::Image(image) => {
                ggraphics::Canvas::draw(&mut canvas, image, renderer.draw_param)
            }
            render_type::RenderType::InstanceArray(_) => todo!(),
            render_type::RenderType::Mesh(_) => todo!(),
            render_type::RenderType::Text(_) => todo!(),
        }
    }
}
