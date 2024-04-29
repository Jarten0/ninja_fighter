use bevy_reflect::{Reflect, ReflectKind, ReflectMut};
use engine::editor::FieldWidget;
use ggez::graphics::{Color, GraphicsContext, Image, InstanceArray, Mesh, Text};
use serde::Serialize;

#[allow(dead_code)]
#[derive(Debug, Default)]
pub enum RenderType {
    Image(Image),
    InstanceArray(InstanceArray),
    Mesh(Mesh),
    Text(Text),
    #[default]
    None,
}

impl FieldWidget for RenderType {
    fn ui(value: &mut dyn bevy_reflect::Reflect, ui: &mut egui::Ui) {
        let ReflectMut::Enum(value) = value.reflect_mut() else {
            ui.label("value is bugged".to_owned() + value.reflect_type_path());
            return;
        };

        egui::ComboBox::from_label("Rendering data type")
            .selected_text("{:#?}")
            .show_ui(ui, |ui: &mut egui::Ui| {
                // ui.selectable_value(value, RenderType::None, "None")
            });
    }
}

impl Serialize for RenderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            RenderType::Image(_) => serializer.serialize_str("image"),

            RenderType::InstanceArray(_) => todo!(),
            RenderType::Mesh(_) => todo!(),
            RenderType::Text(_) => todo!(),
            RenderType::None => todo!(),
        }
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
        }
    }
}
