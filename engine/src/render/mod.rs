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
    // s.serialize_field("transform", &value.transform);
    s.serialize_field("z", &value.z);
    s.end()
}

pub fn deserialize_draw_param<'de, D>(deserializer: D) -> Result<DrawParam, D::Error>
where
    D: serde::Deserializer<'de>,
{
    deserializer.deserialize_struct("DrawParam", &["color", "src", "z"], DrawParamVisitor)
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
        let mut z = None;

        loop {
            if let Some(key) = map.next_key::<String>()? {
                match key.as_str() {
                    "color" => color = Some(map.next_value::<ggez::graphics::Color>()?),
                    "src" => src = Some(map.next_value::<ggez::graphics::Rect>()?),
                    "z" => z = Some(map.next_value::<i32>()?),
                    _ => continue,
                }
            } else {
                break;
            }
        }

        Ok(DrawParam {
            src: src.expect("expected a src field in the map"),
            color: color.expect("expected a color field in the map"),
            transform: ggez::graphics::Transform::default(),
            z: z.expect("expected a z field in the map"),
        })
    }
}

#[cfg(feature = "editor_features")]
pub fn draw_param_ui(ui: &mut egui::Ui, draw_param: &mut DrawParam) {
    ui.collapsing("Draw Param", |ui| {
        ui.collapsing("src", |ui| {
            ui.add(egui::DragValue::new(&mut draw_param.src.x));
            ui.add(egui::DragValue::new(&mut draw_param.src.y));
            ui.add(egui::DragValue::new(&mut draw_param.src.w));
            ui.add(egui::DragValue::new(&mut draw_param.src.h));
        });

        let rgba = draw_param.color.to_rgba();
        let color_label = ui.label("Color");
        let mut color32 = egui::Color32::from_rgba_unmultiplied(rgba.0, rgba.1, rgba.2, rgba.3);
        ui.color_edit_button_srgba(&mut color32)
            .labelled_by(color_label.id);
        draw_param.color(ggez::graphics::Color::from_rgba(
            color32.r(),
            color32.g(),
            color32.b(),
            color32.a(),
        ));

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
        });

        let z_label = ui.label("z");
        ui.add(egui::DragValue::new(&mut draw_param.z))
            .labelled_by(z_label.id);
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
