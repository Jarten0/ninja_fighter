use bevy_ecs::component::Component;
use bevy_ecs::system::{Query, ResMut};
use bevy_reflect::Reflect;
use engine::editor::FieldWidget;
use engine::GgezInterface;
use ggez::graphics;
use serde::{Deserialize, Serialize};

#[derive(Default, Component, Reflect, Serialize, Deserialize)]
pub struct TextRenderer {
    #[reflect(ignore)]
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub text_object: ggez::graphics::Text,

    #[reflect(ignore)]
    #[serde(serialize_with = "engine::render::serialize_draw_param")]
    #[serde(deserialize_with = "engine::render::deserialize_draw_param")]
    draw_param: ggez::graphics::DrawParam,
}

impl FieldWidget for TextRenderer {
    fn ui(value: &mut dyn Reflect, ui: &mut egui::Ui) {
        let field_value = value.downcast_mut::<Self>().unwrap(); //you can use this if your type implements reflect

        for fragment in field_value.text_object.fragments_mut() {
            ui.text_edit_multiline(&mut fragment.text);

            ui.collapsing("Fragment Options", |ui| {
                if let None = fragment.color {
                    fragment.color(graphics::Color::WHITE);
                }

                let mut converter = ColorConverter::ggez(&mut fragment.color);
                ui.color_edit_button_srgba(&mut converter.egui_color);
            });
        }
    }
}

pub struct ColorConverter<'color> {
    ggez_color: &'color mut Option<ggez::graphics::Color>,
    egui_color: egui::Color32,
}

impl<'color> ColorConverter<'color> {
    pub fn ggez(ggez_color: &'color mut Option<graphics::Color>) -> Self {
        let (r, g, b, a) = ggez_color.unwrap().to_rgba();
        Self {
            ggez_color,
            egui_color: egui::Color32::from_rgba_unmultiplied(r, g, b, a),
        }
    }

    pub fn  
    

    // pub fn set_with_ggez()
}

pub fn render_text_renderers(query: Query<&TextRenderer>, mut engine: ResMut<GgezInterface>) {
    for renderer in query.iter() {
        engine
            .get_canvas_mut()
            .expect("expected the text rendering system to be run only in a draw frame")
            .draw(&renderer.text_object, renderer.draw_param)
    }
}
