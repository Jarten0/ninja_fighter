use bevy_reflect::{Struct, TypePath};
use serde::*;

fn main() {
    println!("{}", ReflectableVertex::type_path());
    dbg!(ReflectableVertex::field_at(
        &ReflectableVertex::default(),
        2
    ));
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(remote = "ggez::graphics::Vertex")]
pub struct ReflectableVertex {
    pub position: [f32; 2],
    pub uv: [f32; 2],
    pub color: [f32; 4],
}

bevy_reflect::impl_reflect!(
    #[type_path = "ggez::graphics::Vertex"]
    pub struct ReflectableVertex {
        pub position: [f32; 2],
        pub uv: [f32; 2],
        pub color: [f32; 4],
    }
);
