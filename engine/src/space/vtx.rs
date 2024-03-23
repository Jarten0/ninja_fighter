use std::ops::DerefMut;

use std::ops::Deref;

use bevy_reflect::Reflect;
use ggez::graphics;
use ggez::graphics::Vertex as DrawVertex;
use serde::Deserialize;
use serde::Serialize;

use super::Vector2;

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, Reflect)]
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
            color: [1.0, 0.0, 1.0, 1.0],
        }
    }
}

impl From<DrawVertex> for Vertex {
    fn from(value: DrawVertex) -> Self {
        Self(Vector2 {
            x: value.position[0],
            y: value.position[1],
        })
    }
}

impl From<&DrawVertex> for Vertex {
    fn from(value: &ggez::graphics::Vertex) -> Self {
        let vector2 = Vector2 {
            x: value.position[0],
            y: value.position[1],
        };
        Self(vector2)
    }
}

pub trait ClickableHitbox {}

pub enum ClickableTypes {
    Rect(graphics::Rect),
    Radius(f32),
    Mesh(Box<dyn ClickableHitbox>),
}

pub trait Clickable {
    /// Checks if the mouse is currently hovering over the object.
    ///
    /// It's not just called when it's clicked on, it will be called to check if the mouse is currently able to click on it.
    fn is_clickable(&self, mouse_pos: Vertex) -> bool;

    /// What level of the z-axis is this object on
    ///
    /// If more than one object is clickable, and the z-axis is lower than another clickable object, ignore this object.
    fn z_axis(&self) -> f32 {
        1.0
    }
}
