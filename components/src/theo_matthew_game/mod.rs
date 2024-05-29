use core::panic;

use bevy_ecs::component::Component;
use bevy_ecs::reflect::ReflectComponent;
use bevy_ecs::system::{IntoSystem, Query, ResMut};
use bevy_reflect::Reflect;
use engine::editor::FieldWidget;
use engine::GgezInterface;
use ggez::graphics::{self, TextFragment};
use serde::de::Visitor;
use serde::{Deserialize, Serialize};

use crate::render::draw_param_ui;

#[derive(Default, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct TextRenderer {
    #[reflect(ignore)] //invisible reflection bug AAAAAAAAAAAAAAAAAAAAAAAAAA
    #[serde(serialize_with = "crate::theo_matthew_game::serialize_ggez_text")]
    #[serde(deserialize_with = "crate::theo_matthew_game::deserialize_ggez_text")]
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
                text_fragment_color_ui(fragment, ui);

                draw_param_ui(ui, &mut field_value.draw_param)
            });
        }

        if ui.button("Add text fragmet").clicked() {
            field_value.text_object.add("New text fragment");
        }

        if ui.button("Clear text fragments").clicked() {
            field_value.text_object.clear();
        }
    }
}

fn text_fragment_color_ui(fragment: &mut graphics::TextFragment, ui: &mut egui::Ui) {
    if let None = fragment.color {
        *fragment = fragment.clone().color(graphics::Color::WHITE);
    }

    let mut converter = ColorConverter::from_ggez(fragment.color.unwrap());

    ui.color_edit_button_srgba(&mut converter.egui_color);

    *fragment = fragment.clone().color(converter.convert_to_ggez());
}

pub struct ColorConverter {
    ggez_color: ggez::graphics::Color,
    egui_color: egui::Color32,
}

impl ColorConverter {
    pub fn from_ggez(ggez_color: graphics::Color) -> Self {
        let (r, g, b, a) = ggez_color.to_rgba();
        Self {
            ggez_color,
            egui_color: egui::Color32::from_rgba_unmultiplied(r, g, b, a),
        }
    }

    pub fn from_egui(egui_color: egui::Color32) -> Self {
        let [r, g, b, a] = egui_color.to_srgba_unmultiplied();
        Self {
            ggez_color: graphics::Color::from_rgba(r, g, b, a),
            egui_color,
        }
    }

    pub fn convert_to_ggez(&self) -> graphics::Color {
        let [r, g, b, a] = self.egui_color.to_srgba_unmultiplied();
        graphics::Color::from_rgba(r, g, b, a)
    }

    pub fn convert_to_egui(&self) -> egui::Color32 {
        let (r, g, b, a) = self.ggez_color.to_rgba();
        egui::Color32::from_rgba_unmultiplied(r, g, b, a)
    }
}

pub fn render_text_renderers(query: Query<&TextRenderer>, mut engine: ResMut<GgezInterface>) {
    for renderer in query.iter() {
        engine
            .get_canvas_mut()
            .expect("expected the text rendering system to be run only in a draw frame")
            .draw(&renderer.text_object, renderer.draw_param)
    }
}

pub fn serialize_ggez_text<S>(value: &graphics::Text, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use serde::ser::SerializeStruct;
    let mut s = serializer.serialize_struct("Text Renderer", 1)?;

    let mut serialized_fragments: Vec<(&String, &Option<graphics::Color>)> = value
        .fragments()
        .iter()
        .map(|fragment: &TextFragment| (&fragment.text, &fragment.color))
        .collect::<Vec<(&String, &Option<graphics::Color>)>>();

    s.serialize_field("fragments", &serialized_fragments);

    s.end()
}

pub fn deserialize_ggez_text<'de, D>(deserializer: D) -> Result<graphics::Text, D::Error>
where
    D: serde::Deserializer<'de>,
{
    deserializer.deserialize_struct("ggez::graphics::Text", &["fragments"], TextVisitor)
}

pub(crate) struct TextVisitor;

impl<'de> Visitor<'de> for TextVisitor {
    type Value = graphics::Text;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a ggez::graphics::Text struct")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut fragments = None;

        if let Some((key, value)) =
            map.next_entry::<String, Vec<(String, Option<graphics::Color>)>>()?
        {
            match key.as_str() {
                "fragments" => fragments = Some(value),
                _ => panic!("Unknown key {}", key),
            }
        }
        let mut renderer = graphics::Text::default();

        for (text, color) in fragments.expect("expected a fragment sequence") {
            let fragment = ggez::graphics::TextFragment::new(text).color(match color {
                Some(some) => {
                    log::info!("found color");
                    some
                }
                None => {
                    log::info!("did not found color :(");
                    graphics::Color::WHITE
                }
            });

            renderer.add(fragment);
        }

        Ok(renderer)
    }
}
