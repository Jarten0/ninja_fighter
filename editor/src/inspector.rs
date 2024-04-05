use bevy_ecs::prelude::*;
use engine::{GgezInterface, Input};
use ggez::graphics::{self, Color, DrawParam, FillOptions};
use log::*;

#[derive(Debug, Resource, Default)]
pub struct Inspector {
    enabled: bool,
}

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

    let inspector_rect = graphics::Rect::new(1320.0, 0.0, 600.0, 1080.0);
    let quad = graphics::Mesh::new_rectangle(
        &engine.get_context().gfx,
        graphics::DrawMode::Fill(FillOptions::DEFAULT),
        inspector_rect,
        Color::from_rgba(40, 40, 40, 230),
    )
    .unwrap();

    engine
        .get_canvas_mut()
        .unwrap()
        .draw(&quad, DrawParam::new());
}
