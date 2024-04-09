use std::fmt::Debug;

use super::InspectorDrawInfo;
use super::InspectorElement;
use super::InspectorView;
use bevy_ecs::system::Query;
use bevy_ecs::system::Res;
use engine::Input;
use ggez::graphics;
use ggez::graphics::Color;
use ggez::graphics::DrawParam;
use ggez::mint::Point2;

pub(crate) fn update(mut query: Query<&mut dyn InspectorClickButton>, input: Res<Input>) {
    for ui_buttons in query.iter() {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ButtonType {
    Click,
    Toggle,
}

impl ButtonType {
    pub fn new() -> Self {
        Self::Click
    }
}

impl Default for ButtonType {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum ClickState {
    #[default]
    Idle,
    Hovering,
    Held,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ToggleButtonState {
    ToggledOff(ClickState),
    ToggledOn(ClickState),
}

#[bevy_trait_query::queryable]
pub trait InspectorClickButton {
    fn state(&self) -> &crate::inspector::button::ClickState;

    fn message(&self) -> Option<&ggez::graphics::Text> {
        None
    }

    fn view(&self) -> InspectorView {
        InspectorView::Entities
    }

    fn button_dimensions(&self) -> ggez::graphics::Rect {
        graphics::Rect {
            x: 0.0,
            y: 0.0,
            w: 20.0,
            h: 20.0,
        }
    }
}

impl<T> InspectorElement for T
where
    T: InspectorClickButton + Debug + Sync + Send,
{
    fn get_height(&self) -> f32 {
        self.button_dimensions().y
    }

    fn view(&self) -> InspectorView {
        self.view()
    }

    fn draw(&self, canvas: &mut graphics::Canvas, inspector_draw_info: &mut InspectorDrawInfo) {
        let button_dimensions = self.button_dimensions();
        let button_dest = Point2::from_slice(&[
            inspector_draw_info.next_dest.x + button_dimensions.x,
            inspector_draw_info.next_dest.y + button_dimensions.y,
        ]);
        let param = DrawParam::new()
            .color(Color {
                r: 0.9,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            })
            .clone()
            .dest_rect(button_dimensions)
            .dest(button_dest);

        match self.state() {
            ClickState::Idle => param.color(Color {
                r: 0.9,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            }),
            ClickState::Hovering => param.color(Color {
                r: 1.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            }),
            ClickState::Held => param.color(Color {
                r: 0.7,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            }),
        };

        canvas.draw(&graphics::Quad, param);

        if self.message().is_some() {
            canvas.draw(
                self.message().unwrap(),
                DrawParam::new()
                    .color(Color::WHITE)
                    .clone()
                    .dest(button_dest),
            );
        }
    }
}
