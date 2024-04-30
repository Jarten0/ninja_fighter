use bevy_reflect::{Reflect, ReflectKind, ReflectMut};
use engine::editor::FieldWidget;
use ggez::graphics::{self, Color, GraphicsContext, Image, InstanceArray, Mesh, Quad, Text};
use serde::Serialize;

#[allow(dead_code)]
#[derive(Debug, Default)]
pub enum RenderType {
    Image(Image),

    InstanceArray(InstanceArray),

    Mesh(Mesh),

    Text(Text),

    Quad(Quad),

    #[default]
    None,
}

impl PartialEq for RenderType {
    fn eq(&self, other: &Self) -> bool {
        match self {
            RenderType::Image(..) => {
                let RenderType::Image(..) = other else {
                    return false;
                };
                true
            }
            RenderType::InstanceArray(..) => {
                let RenderType::InstanceArray(..) = other else {
                    return false;
                };
                true
            }
            RenderType::Mesh(..) => {
                let RenderType::Mesh(..) = other else {
                    return false;
                };
                true
            }
            RenderType::Text(..) => {
                let RenderType::Text(..) = other else {
                    return false;
                };
                true
            }
            RenderType::None => {
                let RenderType::None = other else {
                    return false;
                };
                true
            }
            RenderType::Quad(..) => {
                let RenderType::Quad(..) = other else {
                    return false;
                };
                true
            }
        }
    }
}

impl RenderType {
    pub(crate) fn ui(&mut self, ui: &mut egui::Ui) {
        egui::ComboBox::from_label("Rendering data type")
            .selected_text("{:#?}")
            .show_ui(ui, |ui: &mut egui::Ui| {
                ui.selectable_value(self, RenderType::None, "None")
            });
    }
}

impl RenderType {
    pub fn default(gfx: &GraphicsContext) -> Self {
        Self::Image(Image::from_color(gfx, 100, 100, Some(Color::RED)))
    }
}

impl Clone for RenderType {
    fn clone(&self) -> Self {
        match self {
            Self::Image(arg0) => Self::Image(arg0.clone()),
            Self::InstanceArray(_arg0) => todo!(),
            Self::Mesh(arg0) => Self::Mesh(arg0.clone()),
            Self::Text(arg0) => Self::Text(arg0.clone()),
            Self::None => Self::None,
            Self::Quad(..) => Self::Quad(graphics::Quad),
        }
    }
}
