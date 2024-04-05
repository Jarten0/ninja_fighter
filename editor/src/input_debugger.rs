use bevy_ecs::prelude::*;
use ggez::graphics::{self, Color, DrawParam, Rect, TextFragment, Transform};

use crate::{GgezInterface, Input};

#[derive(Debug, Resource, Default)]
pub struct InputDebugger {
    is_active: bool,
}

pub fn draw_debug_information(
    mut debugger: ResMut<InputDebugger>,
    input: Res<Input>,
    mut engine: ResMut<GgezInterface>,
) {
    if input.get_action("debuglog").unwrap().is_just_pressed() {
        debugger.is_active = !debugger.is_active;
    }

    if !debugger.is_active {
        return;
    }

    let mut text = graphics::Text::default();

    let mut fragments: Vec<String> = vec![];
    for (name, action) in input.iter_actions() {
        fragments.push(name.to_string() + ": " + &action.status().to_string() + "\n");
    }

    for fragment in fragments {
        text.add(TextFragment::new(fragment));
    }

    engine.get_canvas_mut().unwrap().draw(
        &text,
        DrawParam {
            src: Rect::default(),
            color: Color::WHITE,
            transform: Transform::default(),
            z: 100,
        },
    )
}
