use bevy_ecs::system::Query;
use ggez::graphics::{
    self as ggraphics, DrawParam, GraphicsContext, Image, InstanceArray, Mesh, Text,
};

use crate::space;

use super::context::WorldInfo;

pub enum RenderType {
    Image(Image),
    InstanceArray(InstanceArray),
    Mesh(Mesh),
    Text(Text),
}

impl RenderType {
    pub fn default(gfx: &GraphicsContext) -> Self {
        Self::Image(Image::from_color(
            gfx,
            100,
            100,
            Some(ggraphics::Color::RED),
        ))
    }
}

#[derive(bevy_ecs::component::Component)]
pub struct Renderer {
    pub image: RenderType,
    pub draw_param: DrawParam,
    pub transform: super::Transform,
    pub offset: space::Position,
}

impl Renderer {
    pub fn set(&mut self, draw_param: DrawParam) {
        self.draw_param = draw_param;
        self.draw_param.transform = self.transform.into();
    }
}

impl Renderer {
    pub fn draw(mut query: Query<(&mut Renderer, &mut WorldInfo)>) {
        for (renderer, mut world_info) in query.iter_mut() {
            println!("Drawing.. :)");

            let canvas = match &mut world_info.game_info.current_canvas {
                Some(canvas) => canvas,
                None => return,
            };

            match &renderer.image {
                RenderType::Image(image) => canvas.draw(image, renderer.draw_param),
                RenderType::InstanceArray(_) => todo!(),
                RenderType::Mesh(_) => todo!(),
                RenderType::Text(_) => todo!(),
            }
        }
    }
}

impl Renderer {
    pub fn default(gfx: &GraphicsContext) -> Self {
        Self {
            image: RenderType::default(gfx),
            draw_param: Default::default(),
            transform: Default::default(),
            offset: Default::default(),
        }
    }
}
