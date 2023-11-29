use ggez::graphics::{Image, InstanceArray, Mesh, Text};

pub enum RenderType {
    Image(Image),
    InstanceArray(InstanceArray),
    Mesh(Mesh),
    Text(Text),
}

#[derive(bevy_ecs::component::Component)]
pub struct Renderer {
    pub image: RenderType,
}
