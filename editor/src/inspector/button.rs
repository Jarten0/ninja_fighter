use super::Inspector;
use super::InspectorElement;
use super::InspectorView;
use bevy_ecs::component::Component;
use bevy_ecs::system::Query;
use bevy_ecs::system::Res;
use bevy_ecs::system::ResMut;
use engine::Input;
use ggez::graphics;
use ggez::graphics::Color;
use ggez::graphics::DrawParam;
use ggez::graphics::Text;
use ggez::mint::Point2;

pub(crate) fn update(mut query: Query<&mut InspectorButton>, input: Res<Input>) {
    for ui_button in query.iter() {
        ui_button;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ButtonType {
    Click(ClickButtonState),
    Toggle(ToggleButtonState),
}

impl ButtonType {
    pub fn new() -> Self {
        Self::Click(ClickButtonState::Idle)
    }
}

impl Default for ButtonType {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum ClickButtonState {
    #[default]
    Idle,
    Hovering,
    Held,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ToggleButtonState {
    ToggledOff(ClickButtonState),
    ToggledOn(ClickButtonState),
}

#[derive(Debug, Clone, Component)]
pub struct InspectorButton {
    pub(crate) state: ButtonType,
    pub(crate) message: Option<graphics::Text>,
    pub(crate) button: graphics::Quad,
}

impl InspectorButton {
    pub(crate) fn new(message: Option<String>) -> InspectorButton {
        let mut text: Option<Text> = None;
        if let Some(message) = message {
            text = Some(graphics::Text::new(graphics::TextFragment::new(message)));
        };

        Self {
            message: text,
            button: graphics::Quad,
            state: ButtonType::new(),
        }
    }
}

impl InspectorElement for InspectorButton {
    fn get_height(&self) -> f32 {
        20.0
    }

    fn view(&self) -> InspectorView {
        InspectorView::Entities
    }

    fn draw(&self, canvas: &mut graphics::Canvas, inspector: &mut ResMut<Inspector>) {
        let param = DrawParam::new()
            .color(Color {
                r: 0.9,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            })
            .clone()
            .dest(Point2 {
                x: 1320.0,
                y: inspector.next_y_position,
            });

        if let ButtonType::Click(state) = self.state.clone() {
            match state {
                ClickButtonState::Idle => param.color(Color {
                    r: 0.9,
                    g: 0.0,
                    b: 0.0,
                    a: 1.0,
                }),
                ClickButtonState::Hovering => param.color(Color {
                    r: 1.0,
                    g: 0.0,
                    b: 0.0,
                    a: 1.0,
                }),
                ClickButtonState::Held => param.color(Color {
                    r: 0.7,
                    g: 0.0,
                    b: 0.0,
                    a: 1.0,
                }),
            };
        }

        canvas.draw(&self.button, param);

        if self.message.is_some() {
            canvas.draw(
                self.message.as_ref().unwrap(),
                DrawParam::new().color(Color::WHITE).clone().dest(Point2 {
                    x: 1340.0,
                    y: inspector.next_y_position,
                }),
            );
        }
    }
}
