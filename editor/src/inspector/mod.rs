use std::fmt::format;

use bevy_ecs::prelude::*;
use bevy_reflect::Reflect;
use egui::{Align2, Ui};
use egui_dock::{DockArea, DockState, Style};
use engine::GgezInterface;
use ggez::graphics::{self, DrawParam};
use log::*;

type Tab = String;

struct MyTabViewer;

impl egui_dock::TabViewer for MyTabViewer {
    type Tab = Tab;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.as_str().into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        ui.label(format!("Content of {tab}"));
    }
}

#[derive(Resource)]
pub struct EditorInterface {
    gui: ggegui::Gui,
    dock_state: egui_dock::DockState<Tab>,
}

impl EditorInterface {
    pub fn new(ctx: &mut ggez::Context) -> Self {
        let tabs = ["tabUno", "tab2", "tabFREE"]
            .map(str::to_string)
            .into_iter()
            .collect();

        Self {
            gui: ggegui::Gui::new(ctx),
            dock_state: DockState::new(tabs),
        }
    }

    fn dock_state_ui(&mut self, ui: &mut Ui) {
        DockArea::new(&mut self.dock_state)
            .style(Style::from_egui(ui.style().as_ref()))
            .show_inside(ui, &mut MyTabViewer);
    }
}

pub fn init_editor_gui(mut editor: ResMut<EditorInterface>, mut engine: ResMut<GgezInterface>) {
    // let ctx = engine.get_context_mut();
    // let gui_ctx = editor.gui.ctx();
    // egui::Window::new("Inspector").show(&gui_ctx, |ui| {
    //     ui.label("a very nice gui :3");
    //     if ui.button("print \"hello world\"").clicked() {
    //         println!("hello world");
    //     }
    //     if ui.button("quit").clicked() {
    //         ctx.request_quit();
    //     }
    // });
}

pub fn update_inspector(mut editor: ResMut<EditorInterface>, mut engine: ResMut<GgezInterface>) {
    let ctx = engine.get_context_mut();
    let gui_ctx = editor.gui.ctx();
    egui::Window::new("UI").show(&gui_ctx, |ui| {
        ui.label("a very nice gui :3");
        if ui.button("print \"hello world\"").clicked() {
            println!("hello world");
        }
        if ui.button("quit").clicked() {
            ctx.request_quit();
        }
    });

    let show = egui::Window::new("Inspector")
        // .anchor(Align2::RIGHT_TOP, [-23.0, 0.0])
        .constrain(false)
        .show(&gui_ctx, |ui| {
            editor.dock_state_ui(ui);
        });

    if let Some(inner) = show {
        // inner.
    }

    editor.gui.update(engine.get_context_mut());
}

pub fn draw_editor_gui(editor: Res<EditorInterface>, mut engine: ResMut<GgezInterface>) {
    engine.get_canvas_mut().unwrap().draw(
        &editor.gui,
        DrawParam::default().dest([0.0, 0.0]).z(10000000),
    );
    // trace!("Drew gui")
}
