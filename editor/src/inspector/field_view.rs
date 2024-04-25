use std::any::Any;
use std::fmt::Debug;
use std::ops::RangeInclusive;

use super::InspectorWindow;
use super::Response;
use bevy_reflect::FromType;
use bevy_reflect::NamedField;
use bevy_reflect::Reflect;
use bevy_reflect::Struct;
use egui::Ui;
use egui::Widget;
use engine::space::Vector2;
use log::trace;

#[derive(Debug, Default)]
pub struct FieldViewState {}

pub(super) fn draw_field(
    state: &mut InspectorWindow,
    ui: &mut egui::Ui,
    tab: &mut <InspectorWindow as egui_dock::TabViewer>::Tab,
) -> Option<Response> {
    ui.label("No implementation of field editor at the moment");
    None
}

/// A field of an inspectable component, stored in the entities [`InspectorData`].
///
/// There must be a [`egui::Widget`] stored with the field for it to display.
pub struct InspectorComponentField {
    pub field_inspection_data: engine::editor::InspectableAsField,
    pub field_name: String,
}

impl Debug for InspectorComponentField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InspectorComponentField")
            .field("field_name", &self.field_name)
            .finish()
    }
}

#[derive(Debug, Reflect)]
pub struct CustomWidget;

impl egui::Widget for CustomWidget {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.label("CustomWidget:D");
        egui::Response {
            ctx: egui::Context::default(),
            layer_id: egui::LayerId::new(egui::Order::Middle, egui::Id::new("CustomWidget:D")),
            id: egui::Id::new("CustomWidget:DD"),
            rect: egui::Rect::from_min_size(egui::Pos2::ZERO, egui::Vec2::new(200.0, 30.0)),
            sense: egui::Sense::focusable_noninteractive(),
            enabled: true,
            contains_pointer: false,
            hovered: false,
            highlighted: false,
            clicked: [false, false, false, false, false],
            double_clicked: [false, false, false, false, false],
            triple_clicked: [false, false, false, false, false],
            drag_started: false,
            dragged: false,
            drag_released: false,
            is_pointer_button_down_on: false,
            interact_pointer_pos: None,
            changed: false,
        }
    }
}
