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
    pub field_inspection_data: InspectableAsField,
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

/// A simple supertrait for `egui::Widget` that requires the type to implement `Sync` and `Send` (also `Debug`)
// #[bevy_reflect::reflect_trait]
pub trait FieldWidget: Send + Sync + Sized {
    fn ui(value: &mut dyn Reflect, ui: &mut Ui) -> egui::Response {
        ui.label("Default implementation of widget for ".to_owned() + value.reflect_type_path())
    }
}

impl FieldWidget for f32 {
    fn ui(value: &mut dyn Reflect, ui: &mut Ui) -> egui::Response {
        let value = value.downcast_mut::<f32>().unwrap();

        ui.add(egui::Slider::new(value, 0.0..=100.0))
    }
}

impl FieldWidget for bool {
    fn ui(value: &mut dyn Reflect, ui: &mut Ui) -> egui::Response {
        let value = value.downcast_mut::<bool>().unwrap();

        ui.add(egui::Checkbox::new(value, "test bool"))
    }
}

impl FieldWidget for engine::space::Vector2 {
    fn ui(value: &mut dyn Reflect, ui: &mut Ui) -> egui::Response {
        let value = value.downcast_mut::<Vector2>().unwrap();

        ui.horizontal_top(|ui| {
            ui.label("x");
            egui::DragValue::new(&mut value.x).ui(ui);
            ui.label("y");
            egui::DragValue::new(&mut value.y).ui(ui)
        })
        .inner
    }
}

impl FieldWidget for engine::space::Position {}

// impl<T> FieldWidget for T where T: Sync + Send {}

/// Insert into the type registry.
///
/// States that the type can be inspected as a field, and when it is, display this widget.
///
/// This also works for structs that contain other inspectable fields.
///
/// You must give it information required to serialize, display and edit it via [`egui`]
#[derive(Debug, Clone)]
pub struct InspectableAsField {
    ui_display_fn: fn(&mut dyn Reflect, &mut Ui) -> egui::Response,
}

impl<T: FieldWidget> FromType<T> for InspectableAsField {
    fn from_type() -> Self {
        Self::new(<T as FieldWidget>::ui)
    }
}

// impl Default for InspectableAsField {
//     fn default() -> Self {
//         Self {
//             ui_display_fn: |field_data, ui| {
//                 ui.add(egui::Label::new("DefaultFieldWidget"));
//             },
//         }
//     }
// }

impl InspectableAsField {
    pub fn new(ui_display_fn: fn(&mut dyn Reflect, &mut Ui) -> egui::Response) -> Self {
        Self { ui_display_fn }
    }

    pub fn show(&self, ui: &mut egui::Ui, field: &mut dyn Reflect) -> egui::Response {
        (self.ui_display_fn)(field, ui)
    }
}
