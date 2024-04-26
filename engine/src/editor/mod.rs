use std::ops::{Deref, DerefMut};

use bevy_reflect::{FromType, Reflect};
use egui::{Ui, Widget};

use crate::space::Vector2;

/// A simple supertrait for `egui::Widget` that requires the type to implement `Sync` and `Send` (also `Debug`)
// #[bevy_reflect::reflect_trait]
pub trait FieldWidget: Send + Sync + Sized + Reflect {
    // the Reflect trait bound might be removed later
    fn ui(value: &mut dyn Reflect, ui: &mut Ui) -> egui::Response {
        ui.label("Default implementation of widget for ".to_owned() + value.reflect_type_path())
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum NumberFieldTypes<T> {
    DragValue,
    Slider { min: T, max: T },
}

impl FieldWidget for f32 {
    fn ui(value: &mut dyn Reflect, ui: &mut Ui) -> egui::Response {
        let value = value.downcast_mut::<f32>().unwrap();

        egui::DragValue::new(value)
            .ui(ui)
            .context_menu(|ui| {
                egui::ComboBox::from_label("Pick a slider type")
                    .selected_text(format!("Waht {:?}"))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            todo!(),
                            NumberFieldTypes::DragValue::<f32>,
                            "Drag value",
                        );
                        ui.selectable_value(
                            todo!(),
                            NumberFieldTypes::Slider {
                                min: -100.0,
                                max: 100.0,
                            },
                            "Slider",
                        );
                    });
            })
            .unwrap()
            .response

        // ui.add(egui::Slider::new(value, 0.0..=100.0))
    }
}

impl FieldWidget for f64 {
    fn ui(value: &mut dyn Reflect, ui: &mut Ui) -> egui::Response {
        let value = value.downcast_mut::<f64>().unwrap();

        ui.add(egui::Slider::new(value, -100.0..=100.0))
    }
}

impl FieldWidget for bool {
    fn ui(value: &mut dyn Reflect, ui: &mut Ui) -> egui::Response {
        let value = value.downcast_mut::<bool>().unwrap();

        ui.add(egui::Checkbox::new(value, "test bool"))
    }
}

impl FieldWidget for Vector2 {
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

impl FieldWidget for crate::space::Position {}

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
