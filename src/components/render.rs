use std::default;

use bevy_ecs::system::Query;
use ggez::{graphics::{Image, InstanceArray, Mesh, Text, Canvas, DrawParam, GraphicsContext}, Context};

use crate::{Draw, DrawBas, GameInfo, space::Position};

use ggez::graphics as ggraphics;

use super::Transform;

pub enum RenderType {
    Image(Image),
    InstanceArray(InstanceArray),
    Mesh(Mesh),
    Text(Text),
}

impl RenderType {
    fn default(gfx: &GraphicsContext) -> Self {
        Self::Image(Image::from_color(gfx, 10, 10, Some(ggraphics::Color::RED)))
    }
}

#[derive(bevy_ecs::component::Component)]
pub struct Renderer {
    pub image: RenderType,
    pub draw_param: DrawParam,
    pub transform: Transform,
    pub offset: Position,
}

// impl Draw<&mut Self> for Renderer {
//     fn draw(mut query: Query<&mut Self>) {
//         for renderer in query.iter_mut() {
            
//         }

//     }
// }

impl Renderer {
    fn set(&mut self) {
        self.draw_param = DrawParam::new();
        self.draw_param.transform = self.transform.into();
    }
}

impl DrawBas for Renderer {
    fn draw_bas(&mut self, game_info: &mut GameInfo, ctx: &mut Context, canvas: &mut Canvas) {
        self.set();
        
        match &self.image {
            RenderType::Image(image) => canvas.draw(image, self.draw_param),
            RenderType::InstanceArray(_) => todo!(),
            RenderType::Mesh(_) => todo!(),
            RenderType::Text(_) => todo!(),
        }
    }
}

impl Renderer {
    fn default(gfx: &GraphicsContext) -> Self {
        Self { 
            image: RenderType::default(gfx), 
            draw_param: Default::default(), 
            transform: Default::default(), 
            offset: Default::default() }
    }
}