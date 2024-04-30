use std::ops::{Deref, DerefMut};

use bevy_reflect::{FromType, Reflect};
use egui::{Ui, Widget};

use crate::space::Vector2;

/// A simple supertrait for `egui::Widget` that requires the type to implement `Sync` and `Send` (also `Debug`)
// #[bevy_reflect::reflect_trait]
#[cfg(feature = "editor_features")]
pub trait FieldWidget: Send + Sync + Sized {
    fn ui(value: &mut dyn Reflect, ui: &mut egui::Ui) {
        // let field_value = value.downcast_mut::<Self>().unwrap(); //you can use this if your type implements reflect

        ui.label("Default implementation of widget for ".to_owned() + value.reflect_type_path());
    }
}

impl FieldWidget for f32 {
    fn ui(value: &mut dyn Reflect, ui: &mut Ui) {
        let value = value.downcast_mut::<f32>().unwrap();

        let response = egui::DragValue::new(value).speed(0.05).ui(ui);

        response.context_menu(|ui| {
            ui.button("hehe :3");
        });
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

        ui.add(egui::Checkbox::without_text(value));
    }
}

impl FieldWidget for String {
    fn ui(value: &mut dyn Reflect, ui: &mut egui::Ui) {
        let field_value = value.downcast_mut::<Self>().unwrap(); //you can use this if your type implements reflect

        ui.add(egui::TextEdit::multiline(field_value));
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

// impl FieldWidget for ggez::graphics::DrawParam {
//     fn ui(value: &mut dyn Reflect, ui: &mut egui::Ui) {
//         // let field_value = value.downcast_mut::<Self>().unwrap(); //you can use this if your type implements reflect

//         ui.label("Default implementation of widget for ".to_owned() + value.reflect_type_path());
//         ui.collapsing("Draw Param", |ui| {
//             let draw_param = &mut field_value.draw_param;

//             ui.collapsing("src", |ui| {
//                 ui.add(egui::DragValue::new(&mut draw_param.src.x));
//                 ui.add(egui::DragValue::new(&mut draw_param.src.y));
//                 ui.add(egui::DragValue::new(&mut draw_param.src.w));
//                 ui.add(egui::DragValue::new(&mut draw_param.src.h));
//             });

//             let rgba = draw_param.color.to_rgba();
//             let color_label = ui.label("Color");
//             let mut color32 = egui::Color32::from_rgba_unmultiplied(rgba.0, rgba.1, rgba.2, rgba.3);
//             ui.color_edit_button_srgba(&mut color32)
//                 .labelled_by(color_label.id);
//             draw_param.color(ggraphics::Color::from_rgba(
//                 color32.r(),
//                 color32.g(),
//                 color32.b(),
//                 color32.a(),
//             ));

//             ui.collapsing("Transform", |ui| match &mut draw_param.transform {
//                 ggraphics::Transform::Values {
//                     dest,
//                     rotation,
//                     scale,
//                     offset,
//                 } => {
//                     ui.add(egui::DragValue::new(&mut dest.x));
//                     ui.add(egui::DragValue::new(&mut dest.y));
//                     ui.add(egui::DragValue::new(&mut scale.x));
//                     ui.add(egui::DragValue::new(&mut scale.y));
//                 }
//                 ggraphics::Transform::Matrix(matrix) => {
//                     ui.label("transform is matrix, which is currently unsupported");
//                 }
//             });

//             let z_label = ui.label("z");
//             ui.add(egui::DragValue::new(&mut draw_param.z))
//                 .labelled_by(z_label.id);
//         });
//     }
// }

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

impl InspectableAsField {
    pub fn new(ui_display_fn: fn(&mut dyn Reflect, &mut Ui)) -> Self {
        Self { ui_display_fn }
    }

    pub fn show(&self, ui: &mut egui::Ui, field: &mut dyn Reflect) {
        (self.ui_display_fn)(field, ui)
    }
}
