use bevy_ecs::{
    bundle::Bundle,
    component::Component,
    reflect::ReflectComponent,
    world::{EntityWorldMut, FromWorld, World},
};
use bevy_reflect::{
    impl_reflect, std_traits::ReflectDefault, FromType, Reflect, ReflectDeserialize,
    ReflectSerialize, TypeRegistry,
};
use ggez::graphics::{DrawParam, Rect};
use serde::{de::Visitor, Deserialize, Serialize};
pub mod render_type;

pub fn serialize_draw_param<S>(value: &DrawParam, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use serde::ser::SerializeStruct;
    let mut s = serializer.serialize_struct("DrawParam", 3)?;
    s.serialize_field("src", &value.src);
    s.serialize_field("color", &value.color);
    match &value.transform {
        ggez::graphics::Transform::Values { dest, rotation, scale, offset } => {
            let value  = ((dest.x, dest.y), rotation.clone(), (scale.x, scale.y), (offset.x, offset.y));
            s.serialize_field("transform", &Some(value))
        }
        ggez::graphics::Transform::Matrix(matrix) => s.serialize_field("transform", &Option::<((f32, f32), f32, (f32, f32), (f32, f32))>::None),
    }?;

    s.serialize_field("z", &value.z);
    s.end()
}

pub fn deserialize_draw_param<'de, D>(deserializer: D) -> Result<DrawParam, D::Error>
where
    D: serde::Deserializer<'de>,
{
    deserializer.deserialize_struct("DrawParam", &["color", "src", "transform", "z"], DrawParamVisitor)
}

pub(crate) struct DrawParamVisitor;

impl<'de> Visitor<'de> for DrawParamVisitor {
    type Value = DrawParam;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a DrawParam struct")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut color = None;
        let mut src = None;
        let mut transform: Option<Option<((f32, f32), f32, (f32, f32), (f32, f32))>> = None;
        let mut z = None;

        loop {
            if let Some(key) = map.next_key::<String>()? {
                match key.as_str() {
                    "color" => color = Some(map.next_value::<ggez::graphics::Color>()?),
                    "src" => src = Some(map.next_value::<ggez::graphics::Rect>()?),
                    "z" => z = Some(map.next_value::<i32>()?),
                    "transform" => transform = Some(map.next_value()?),
                    _ => continue,
                }
            } else {
                break;
            }
        }

        let transform = match transform.expect("expected a transform field in the map, even if transform is null") {
            Some(some) => ggez::graphics::Transform::Values { dest: ggez::glam::Vec2::from(some.0).into(), rotation: some.1, scale: ggez::glam::Vec2::from(some.2).into(), offset: ggez::glam::Vec2::from(some.3).into() },
            None => {
                log::error!("Transform was deserialized as None, did you serialize a matrix transform?");
                ggez::graphics::Transform::default()},
        };

        Ok(DrawParam {
            src: src.expect("expected a src field in the map"),
            color: color.expect("expected a color field in the map"),
            transform,
            z: z.expect("expected a z field in the map"),
        })
    }
}

#[cfg(feature = "editor_features")]
pub fn draw_param_ui(ui: &mut egui::Ui, draw_param: &mut DrawParam) {
    ui.collapsing("Draw Param", |ui| {
        ui.collapsing("Transform", |ui| match &mut draw_param.transform {
            ggez::graphics::Transform::Values {
                dest,
                rotation,
                scale,
                offset,
            } => {
                ui.add(egui::DragValue::new(&mut dest.x));
                ui.add(egui::DragValue::new(&mut dest.y));
                ui.add(egui::DragValue::new(&mut scale.x));
                ui.add(egui::DragValue::new(&mut scale.y));
            }
            ggez::graphics::Transform::Matrix(matrix) => {
                ui.label("transform is matrix, which is currently unsupported");
            }
        }).header_response.on_hover_text("Where the object will be rendered on the screen.
Rendering code should not edit this, instead creatinng a new DrawParam with a new dest to move an object
accordingly to factors outside of the renderer itself, such as compensation for the camera.");

        ui.collapsing("src", |ui| {
            ui.add(egui::DragValue::new(&mut draw_param.src.x));
            ui.add(egui::DragValue::new(&mut draw_param.src.y));
            ui.add(egui::DragValue::new(&mut draw_param.src.w));
            ui.add(egui::DragValue::new(&mut draw_param.src.h));
        })
        .header_response
        .on_hover_text(
            "A portion of the drawable to clip, as a fraction of the whole image.
Defaults to the whole image ([0.0, 0.0] to [1.0, 1.0]) if omitted.",
        );

        let z_label = ui.label("z");
        ui.add(egui::DragValue::new(&mut draw_param.z))
            .labelled_by(z_label.id).on_hover_text("The layer in which the object will be rendered according to the canvas.");

        let color_tooltip = "A color picker for setting the color of this drawparam.
        Note that this may go unused by rendering code.";
        
        
        let color_label = ui.label("Color").on_hover_text(color_tooltip);

        let color = draw_param.color;
        
        let mut rgba = egui::Rgba::from_rgba_unmultiplied(color.r, color.g, color.b, color.a);
        egui::color_picker::color_edit_button_rgba(ui, &mut rgba, egui::color_picker::Alpha::Opaque);
        draw_param.color = rgba.to_array().into();

        
        
        
    });
}

#[derive(Debug, Clone)]
pub struct DowncastInsert {
    downcast_insert: fn(&mut EntityWorldMut, Box<dyn Reflect>),
}

impl DowncastInsert {
    /// [`ReflectComponent`] is typically used to dynamically insert components via reflection,
    /// but it only gives options to insert via applying, which ignores fields that cannot be reflected.
    ///
    /// Thus, this exists as an alternative that downcasts the object, which allows one to access unreflectable fields (hopefully, am testing)
    pub fn downcast_insert(&self, entity: &mut EntityWorldMut, component: Box<dyn Reflect>) {
        (self.downcast_insert)(entity, component)
    }
}

impl<T: Component + Bundle + Reflect> FromType<T> for DowncastInsert {
    fn from_type() -> Self {
        Self {
            downcast_insert: |entity, component| {
                entity.insert(
                    *component
                        .downcast::<T>()
                        .expect("expected component to be of type _"),
                ); //todo: give a proper error msg
            },
        }
    }
}
