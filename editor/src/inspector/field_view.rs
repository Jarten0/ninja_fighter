use std::any::Any;
use std::fmt::Debug;

use super::InspectorWindow;
use super::Response;
use bevy_reflect::FromType;
use bevy_reflect::NamedField;
use bevy_reflect::Reflect;
use egui::Ui;
use egui::Widget;

#[derive(Debug, Default)]
pub struct FieldViewState {}

pub fn draw_field(
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
    pub field_widget: Box<dyn FieldWidget>,
    pub field_name: String,
}

impl Debug for InspectorComponentField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InspectorComponentField")
            .field("field_widget", &None::<InspectorComponentField>)
            .field("field_name", &self.field_name)
            .finish()
    }
}

#[derive(Debug, Reflect)]
pub struct CustomWidget;

impl egui::Widget for CustomWidget {
    fn ui(self, ui: &mut Ui) -> egui::Response {
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
#[bevy_reflect::reflect_trait]
pub trait FieldWidget: egui::Widget + Send + Sync {}
impl<T> FieldWidget for T where T: Sync + Send + egui::Widget {}

impl egui::Widget for Box<dyn FieldWidget> {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        ui.add(self)
    }
}

/// Insert into the type registry.
///
/// States that the type can be inspected as a field, and when it is, display this widget.
///
/// This also works for structs that contain other inspectable fields.
///
/// You must give it information required to serialize, display and edit it via [`egui`]
#[derive(Debug, Clone)]
pub struct InspectableAsField {
    custom_widget_fn: fn() -> Box<dyn FieldWidget>,
}

pub trait Inspectable {
    fn widget() -> Box<dyn FieldWidget>
    where
        Self: Sized;
}

impl<T> FromType<T> for InspectableAsField {
    fn from_type() -> Self {
        Self::default()
    }
}

impl Default for InspectableAsField {
    fn default() -> Self {
        Self {
            custom_widget_fn: || {
                Box::new(egui::Label::new("DefaultFieldWidget")) as Box<dyn FieldWidget>
            },
        }
    }
}

impl InspectableAsField {
    pub fn new(custom_widget_fn: fn() -> Box<dyn FieldWidget>) -> Self {
        Self { custom_widget_fn }
    }

    pub fn create_widget(&self) -> Box<dyn FieldWidget> {
        (self.custom_widget_fn)()
    }
}
