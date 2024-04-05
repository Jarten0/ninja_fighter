use std::{collections::HashMap, default, fmt::Debug};

use bevy_ecs::{prelude::*, query};
use engine::{scene::ObjectID, GgezInterface, Input};
use ggez::{
    graphics::{self, Color, DrawParam, Drawable, FillOptions, Text},
    mint::Point2,
};
use log::*;

#[derive(Debug, Resource)]
pub struct Inspector {
    enabled: bool,
    width: f32,
    view: InspectorView,
    elements: HashMap<ObjectID, Box<dyn InspectorElement>>,
}

impl Default for Inspector {
    fn default() -> Self {
        Self {
            enabled: false,
            width: 600.0,
            view: InspectorView::default(),
            elements: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum InspectorView {
    #[default]
    Entities,
    Components,
    DebugInfo,
    Misc,
}

pub trait InspectorElement
where
    Self: Send + Sync + Debug,
{
    /// How tall is this element in the inspector, at this current frame
    fn get_height(&self) -> f32;

    /// Which inspector view does this element belong to
    fn view(&self) -> InspectorView;
}

pub fn update_inspector() {}

pub fn draw_inspector(
    mut inspector: ResMut<Inspector>,
    mut engine: ResMut<GgezInterface>,
    input: Res<Input>,
) {
    if input
        .get_action("enableinspector")
        .unwrap()
        .is_just_pressed()
    {
        match inspector.enabled {
            false => {
                inspector.enabled = true;
                debug!("Enabled inspector");
            }
            true => {
                inspector.enabled = false;
                debug!("Disabled inspector");
            }
        }
    }

    if !inspector.enabled {
        return;
    }

    engine.get_context().gfx.window().set_maximized(true);

    let inspector_rect = graphics::Rect::new(1920.0 - inspector.width, 0.0, 600.0, 1060.0);
    let quad = graphics::Mesh::new_rectangle(
        &engine.get_context().gfx,
        graphics::DrawMode::Fill(FillOptions::DEFAULT),
        inspector_rect,
        Color::from_rgba(40, 40, 40, 230),
    )
    .unwrap();

    quad.draw(engine.get_canvas_mut().unwrap(), DrawParam::new());

    match inspector.view {
        InspectorView::Entities => inspector_draw_entities(inspector, engine, input),
        InspectorView::Components => todo!(),
        InspectorView::DebugInfo => todo!(),
        InspectorView::Misc => todo!(),
    }
}

fn inspector_draw_entities(
    mut inspector: ResMut<Inspector>,
    mut engine: ResMut<GgezInterface>,
    input: Res<Input>,
) {
    let mut y = 0.0;
    for (id, element) in &inspector.elements {
        if !(element.view() == InspectorView::Entities) {
            continue;
        }
        element.draw(engine.get_canvas_mut().unwrap(), y);
        y += element.get_height();
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
pub struct UnlabeledInspectorButton {
    state: ButtonType,

    message: Option<graphics::Text>,
    button: graphics::Quad,
}

impl UnlabeledInspectorButton {
    fn new(message: Option<String>) -> UnlabeledInspectorButton {
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

impl InspectorElement for UnlabeledInspectorButton {
    fn get_height(&self) -> f32 {
        20.0
    }

    fn view(&self) -> InspectorView {
        InspectorView::Entities
    }
}
fn update(mut query: Query<&mut UnlabeledInspectorButton>, input: Res<Input>) {
    for ui_button in query.iter() {
        todo!()
    }
}

fn draw(&self, canvas: &mut graphics::Canvas, y_offset: f32) {
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
            y: y_offset,
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
                y: y_offset,
            }),
        );
    }
}
