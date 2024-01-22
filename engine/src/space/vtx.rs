use std::ops::DerefMut;

use std::ops::Deref;

use ggez::graphics::Vertex as DrawVertex;
use serde::Deserialize;
use serde::Serialize;

use super::Vector2;

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct Vertex(Vector2);

impl Deref for Vertex {
    type Target = Vector2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Vertex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vector2> for Vertex {
    fn from(value: Vector2) -> Self {
        Self { 0: value }
    }
}

impl Into<DrawVertex> for Vertex {
    fn into(self) -> DrawVertex {
        DrawVertex {
            position: [self.x, self.y],
            uv: [10.0, 10.0],
            color: [0.0, 0.0, 0.0, 1.0],
        }
    }
}
