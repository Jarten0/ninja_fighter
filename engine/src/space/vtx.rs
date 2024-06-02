use std::ops::DerefMut;

use std::ops::Deref;

use bevy_reflect::Reflect;
use ggez::graphics::Vertex as DrawVertex;
use serde::Deserialize;
use serde::Serialize;

#[cfg(feature = "editor_features")]
use crate::editor::FieldWidget;

use super::Vector2;

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, Reflect)]
pub struct Vertex(Vector2);

#[cfg(feature = "editor_features")]
impl FieldWidget for Vertex {
    fn ui(value: &mut dyn Reflect, ui: &mut egui::Ui) {
        let value = value.downcast_mut::<Self>().unwrap(); //you can use this if your type implements reflect

        ui.horizontal(|ui| {
            ui.add(egui::DragValue::new(&mut value.0.x));
            ui.add(egui::DragValue::new(&mut value.0.y));
        });
        // ui.label("Default implementation of widget for ".to_owned() + value.reflect_type_path());
    }
}

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

impl From<(f32, f32)> for Vertex {
    fn from(value: (f32, f32)) -> Self {
        Self { 0: value.into() }
    }
}

impl From<(i32, i32)> for Vertex {
    fn from(value: (i32, i32)) -> Self {
        Self { 0: value.into() }
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

impl Into<mint::Point2<f32>> for Vertex {
    fn into(self) -> mint::Point2<f32> {
        mint::Point2 {
            x: self.x,
            y: self.y,
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
