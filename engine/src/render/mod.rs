use bevy_reflect::{std_traits::ReflectDefault, ReflectDeserialize, ReflectSerialize};
use ggez::graphics::{DrawParam, Rect};
use serde::{de::Visitor, Deserialize, Serialize};

pub mod render_type;

#[derive(Debug, Serialize, Deserialize)]
pub struct Test {
    #[serde(serialize_with = "crate::render::serialize_draw_param")]
    #[serde(deserialize_with = "crate::render::deserialize_draw_param")]
    field: DrawParam,
}

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
    deserializer.deserialize_struct("DrawParam", &["src", "color", "z"], DrawParamVisitor)
}

pub(crate) struct DrawParamVisitor;

impl<'de> Visitor<'de> for DrawParamVisitor {
    type Value = DrawParam;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a DrawParam struct")
    }
}
