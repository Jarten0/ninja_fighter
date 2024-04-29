use std::ops::{Deref, DerefMut};

use bevy_reflect::{FromType, Reflect};
use egui::{Ui, Widget};

use crate::space::Vector2;

/// A simple supertrait for `egui::Widget` that requires the type to implement `Sync` and `Send` (also `Debug`)
// #[bevy_reflect::reflect_trait]
#[cfg(feature = "editor_features")]
pub trait FieldWidget: Send + Sync + Sized {
    // the Reflect trait bound might be removed later
    fn ui(value: &mut dyn Reflect, ui: &mut egui::Ui) {
        // let field_value = value.downcast_mut::<Self>().unwrap(); //you can use this if your type implements reflect

        ui.label("Default implementation of widget for ".to_owned() + value.reflect_type_path());
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum NumberFieldTypes<T> {
    DragValue,
    Slider { min: T, max: T },
}

impl FieldWidget for f32 {
    fn ui(value: &mut dyn Reflect, ui: &mut Ui) {
        let value = value.downcast_mut::<f32>().unwrap();

        egui::DragValue::new(value).speed(0.05).ui(ui);

        // ui.add(
        //     egui::Slider::new(value, -100.0..=100.0)
        //         .logarithmic(false)
        //         .trailing_fill(true)
        //         .max_decimals(2)
        //         .clamp_to_range(false),
        // );
    }
}

impl FieldWidget for f64 {
    fn ui(value: &mut dyn Reflect, ui: &mut Ui) {
        let value = value.downcast_mut::<f64>().unwrap();

        ui.add(egui::Slider::new(value, -100.0..=100.0));
    }
}

impl FieldWidget for bool {
    fn ui(value: &mut dyn Reflect, ui: &mut Ui) {
        let value = value.downcast_mut::<bool>().unwrap();

        ui.add(egui::Checkbox::new(value, "test bool"));
    }
}

impl FieldWidget for Vector2 {
    fn ui(value: &mut dyn Reflect, ui: &mut Ui) {
        let value = value.downcast_mut::<Vector2>().unwrap();

        ui.horizontal_top(|ui| {
            egui::DragValue::new(&mut value.x).prefix("x: ").ui(ui);
            egui::DragValue::new(&mut value.y).prefix("y: ").ui(ui)
        });
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
    ui_display_fn: fn(&mut dyn Reflect, &mut Ui),
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
    pub fn new(ui_display_fn: fn(&mut dyn Reflect, &mut Ui)) -> Self {
        Self { ui_display_fn }
    }

    pub fn show(&self, ui: &mut egui::Ui, field: &mut dyn Reflect) {
        (self.ui_display_fn)(field, ui)
    }
}
