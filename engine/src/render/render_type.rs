use ggez::graphics::{Color, GraphicsContext, Image, InstanceArray, Mesh, Text};

#[allow(dead_code)]
#[derive(Debug)]
pub enum RenderType {
    Image(Image),
    InstanceArray(InstanceArray),
    Mesh(Mesh),
    Text(Text),
}

impl RenderType {
    pub fn default(gfx: &GraphicsContext) -> Self {
        Self::Image(Image::from_color(gfx, 100, 100, Some(Color::RED)))
    }
}
