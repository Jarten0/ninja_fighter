use core::panic;

use bevy_ecs::component::Component;
use bevy_ecs::reflect::ReflectComponent;
use bevy_ecs::system::{IntoSystem, Query, ResMut};
use bevy_reflect::Reflect;
use engine::editor::FieldWidget;
use engine::GgezInterface;
use ggez::graphics::{self, PxScale, TextFragment};
use ggez::GameError;
use serde::de::Visitor;
use serde::{Deserialize, Serialize};

use engine::render::draw_param_ui;

#[derive(Default, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct TextRenderer {
    #[serde(serialize_with = "crate::theo_matthew_game::serialize_ggez_text")]
    #[serde(deserialize_with = "crate::theo_matthew_game::deserialize_ggez_text")]
    #[reflect(ignore)]
    pub text_object: ggez::graphics::Text,

    #[serde(serialize_with = "engine::render::serialize_draw_param")]
    #[serde(deserialize_with = "engine::render::deserialize_draw_param")]
    #[reflect(ignore)]
    draw_param: ggez::graphics::DrawParam,

    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    #[reflect(ignore)]
    fragment_text_buffer: String,
}

impl FieldWidget for TextRenderer {
    fn ui(value: &mut dyn Reflect, ui: &mut egui::Ui) {
        let field_value = value.downcast_mut::<Self>().unwrap(); //you can use this if your type implements reflect

        ui.collapsing("Text Fragments", |ui| {
            for (index, fragment) in field_value
                .text_object
                .fragments_mut()
                .iter_mut()
                .enumerate()
            {
                ui.text_edit_multiline(&mut fragment.text);

                egui::CollapsingHeader::new("Fragment Options")
                    .id_source(("Fragment Options", index))
                    .show(ui, |ui| {
                        text_fragment_ui(&mut field_value.fragment_text_buffer, fragment, ui);
                    });
            }

            if ui.button("Add text fragmet").clicked() {
                field_value.text_object.add("New text fragment");
            }

            if ui.button("Clear text fragments").clicked() {
                field_value.text_object.clear();
            }
        });

        draw_param_ui(ui, &mut field_value.draw_param);
    }
}

fn text_fragment_ui(
    font_text_buffer: &mut String,
    fragment: &mut graphics::TextFragment,
    ui: &mut egui::Ui,
) {
    fragment_color_ui(fragment, ui);

    fragment_scale_ui(fragment, ui);

    // let response = &egui::TextEdit::singleline(font_text_buffer)
    //     .show(ui)
    //     .response;

    // if response.gained_focus() {
    //     *font_text_buffer = match &fragment.font {
    //         Some(some) => some.to_owned(),
    //         None => "".to_string(),
    //     }
    // }

    // if response.lost_focus() {
    //     *fragment = fragment.clone().font(font_text_buffer.clone());
    // }
}

fn fragment_scale_ui(fragment: &mut TextFragment, ui: &mut egui::Ui) {
    //scale ui
    let Some(scale) = &mut fragment.scale else {
        if ui.button("Give unique scale").clicked() {
            *fragment = fragment.clone().scale(20.0);
        }
        return;
    };

    ui.horizontal(|ui| {
        ui.label("Scale");
        ui.add(egui::DragValue::new(&mut scale.x));
        ui.add(egui::DragValue::new(&mut scale.y));
    });
}

fn fragment_color_ui(fragment: &mut TextFragment, ui: &mut egui::Ui) {
    //color ui
    let Some(color) = fragment.color else {
        if ui.button("Give unique color").clicked() {
            *fragment = fragment.clone().color(graphics::Color::WHITE);
        }
        return;
    };

    let mut converter = ColorConverter::from_ggez(color);

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

static DEFAULT_FONT: &str = "segoesc.ttf";
pub fn render_text_renderers(
    mut query: Query<&mut TextRenderer>,
    mut engine: ResMut<GgezInterface>,
) {
    if let Some(error) = engine.error_log.last() {
        if let GameError::FontSelectError(font_name) = error {
            for mut renderer in query.iter_mut() {
                renderer.text_object.set_font(DEFAULT_FONT);
                renderer
                    .text_object
                    .fragments_mut()
                    .iter_mut()
                    .map(|fragment| fragment.font = None);
            }
            engine.error_log.pop();
        }
    }

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

    let mut serialized_fragments = value
        .fragments()
        .iter()
        .map(|fragment: &TextFragment| {
            let scale = match fragment.scale {
                Some(scale) => Some((scale.x, scale.y)),
                None => None,
            };
            (&fragment.text, &fragment.color, scale, &fragment.font)
        })
        .collect::<Vec<(
            &String,
            &Option<graphics::Color>,
            Option<(f32, f32)>,
            &Option<String>,
        )>>();

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

        if let Some((key, value)) = map.next_entry::<String, Vec<(
            String,
            Option<graphics::Color>,
            Option<(f32, f32)>,
            Option<String>,
        )>>()? {
            match key.as_str() {
                "fragments" => fragments = Some(value),
                _ => panic!("Unknown key {}", key),
            }
        }
        let mut renderer = graphics::Text::default();

        for (text, color, scale, font) in fragments.expect("expected a fragment sequence") {
            renderer.add(create_fragment(text, color, scale, font));
        }

        Ok(renderer)
    }
}

fn create_fragment(
    text: String,
    color: Option<graphics::Color>,
    scale: Option<(f32, f32)>,
    font: Option<String>,
) -> TextFragment {
    let mut fragment = ggez::graphics::TextFragment::new(text);

    fragment = (|| {
        let Some(color) = color else {
            return fragment;
        };
        fragment.color(color)
    })();

    fragment = (|| {
        let Some(some) = scale else { return fragment };

        fragment.scale(PxScale {
            x: some.0,
            y: some.1,
        })
    })();

    fragment = (|| {
        let Some(font) = font else { return fragment };

        fragment.font(font)
    })();

    fragment
}
