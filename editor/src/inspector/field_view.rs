use super::InspectorWindow;
use super::Response;
use egui::Ui;

#[derive(Debug, Default)]
pub struct FieldViewState {}

pub fn draw_field(
    ui: &mut egui::Ui,
    tab: &mut <InspectorWindow as egui_dock::TabViewer>::Tab,
) -> Option<Response> {
    ui.label("No implementation of field editor at the moment");
    None
}
