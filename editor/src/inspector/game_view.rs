use std::any::{Any, TypeId};
use std::ops::{Deref, DerefMut};

use bevy_ecs::system::{Res, ResMut};
use bevy_ecs::world::{Mut, World};
use bevy_reflect::TypePath;
use engine::schedule::ScheduleTag;
use engine::GgezInterface;
use ggez::graphics::{self, DrawParam};
use log::trace;

use crate::inspector::EditorTab;
use engine::editor::*;

use super::EditorGUI;

static GAME_VIEW_TAB_ID: &str = "Game View";

#[derive(Debug, Default)]
pub struct GameView {
    current_image: Option<ggez::graphics::Image>,
}

impl super::EditorTab for GameView {
    fn name() -> &'static str
    where
        Self: Sized,
    {
        GAME_VIEW_TAB_ID
    }

    fn display_name(&self) -> String {
        "Game View".to_string()
    }

    fn ui(&mut self, window_state: &mut WindowState, ui: &mut egui::Ui) -> Option<TabResponse> {
        // let Some(canvas) = ggez_interface.get_canvas().cloned() else {
        //     ui.centered_and_justified(|ui| ui.label("Canvas not found."));
        // };

        let width = ui.available_height().round() as u32;
        let height = ui.available_width().round() as u32;

        let game_view_image = ggez::graphics::Image::new_canvas_image(
            window_state
                .world_mut()
                .resource_mut::<GgezInterface>()
                .get_context_mut(),
            ggez::graphics::ImageFormat::Rgba8Unorm,
            width,
            height,
            1,
        );

        let resource = &window_state.world_ref().resource::<GgezInterface>();

        assert!(resource.get_canvas().is_none());

        let canvas = ggez::graphics::Canvas::from_image(
            resource.get_context(),
            game_view_image.clone(),
            ggez::graphics::Color {
                r: 0.1,
                g: 1.0,
                b: 0.1,
                a: 1.0,
            },
        );

        window_state
            .world_mut()
            .resource_mut::<GgezInterface>()
            .set_canvas(canvas);

        window_state.world_mut().run_schedule(ScheduleTag::Frame);

        let resource_mut = &mut window_state.world_mut().resource_mut::<GgezInterface>();

        let Some(mut canvas) = resource_mut.take_canvas() else {
            log::error!("No canvas found!");
            ui.label("Canvas was taken during rendering logic and wasn't set back.");
            return None;
        };

        // canvas.set_screen_coordinates(graphics::Rect {
        //     x: 10.0,
        //     y: 10.0,
        //     w: 362.0,
        //     h: 328.0,
        // });

        // dbg!(canvas.screen_coordinates());
        // dbg!(canvas.scissor_rect());

        self.current_image = Some(game_view_image);
        dbg!(canvas);

        trace!("New image was set");

        // if debug_mode {
        // self.world.run_schedule(ScheduleTag::DebugFrame);
        // }

        // window_state
        //     .world_mut()
        //     .resource_mut::<GgezInterface>()
        //     .get_canvas_mut()
        //     .unwrap()
        //     .draw(&game_view_image, DrawParam::default().z(1001));

        // canvas.finish(
        //     window_state
        //         .world_mut()
        //         .resource_mut::<GgezInterface>()
        //         .get_context_mut(),
        // );

        None
    }

    fn draw(&self, window_state: &WindowState, engine: &mut GgezInterface) {
        if let Some(image) = &self.current_image {
            trace!("aha");
            engine.get_canvas_mut().unwrap().draw(
                image,
                DrawParam::default().z(150000).color(graphics::Color {
                    r: 1.0,
                    g: 0.5,
                    b: 0.0,
                    a: 1.0,
                }),
            )
        }
    }
}

/// Draws the game view into the area set in the [`GameView`] tab.
///
/// Should be ran in the
///
/// What should be drawn in the game view:
/// * Regular game renderers
/// * Optionally, gizmos
///
/// What should not be drawn in the game view:
/// * Any of the editor GUI
/// * The game view window
pub fn draw_editor_views(world: &mut World) {
    unsafe { engine::editor::set_world_raw_pointer(Some(world)) };
    world.resource_scope(|world: &mut World, editor_gui: Mut<EditorGUI>| {
        world.resource_scope(|world: &mut World, mut engine: Mut<GgezInterface>| {
            let editor_gui = editor_gui.deref();
            for ((surface, node), game_view_tab) in editor_gui.dock_state.iter_all_tabs() {
                game_view_tab
                    .state
                    .draw(&editor_gui.window_state, engine.deref_mut());
            }
        });
    });
    unsafe { engine::editor::set_world_raw_pointer(None) };
}
