use super::InspectorWindow;
use super::Response;
use egui::Ui;

#[derive(Debug, Default)]
pub struct InspectorViewState {
    adding_component: bool,
}

pub fn draw_inspector(
    ui: &mut egui::Ui,
    tab: &mut <InspectorWindow as egui_dock::TabViewer>::Tab,
) -> Option<Response> {
    if state.focused_entity.is_none() {
        ui.label("No entity in focus");
        return None;
    }

    ui.label("Inspecting ".to_owned() + &state.focused_entity.as_ref().unwrap().1);
    for (id, component) in state
        .components
        .get(&state.focused_entity.as_ref().unwrap().0)
        .unwrap()
    {
        ui.add(egui::widgets::Button::new(component));
    }
    if ui.button("Add component").clicked() {
        state.inspector.adding_component = !state.inspector.adding_component;
    }
    if state.inspector.adding_component {}
    None
}
