use super::component;
use super::traits::SceneData;
use bevy_ecs::reflect::ReflectComponent;
use bevy_ecs::world::World;
use bevy_reflect::DynamicStruct;
use bevy_reflect::Reflect;
use bevy_reflect::Struct;
use bevy_reflect::TypeRegistry;
use serde::de::Visitor;
use serde::ser::SerializeStruct;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

pub type DataHashmap = HashMap<String, EntityHashmap>;
pub type EntityHashmap = HashMap<String, ComponentHashmap>;
pub type ComponentHashmap = HashMap<String, Value>;

/// Private trait that converts [`serde_json::Value`] to [`Reflect`]
trait ToReflect {
    /// Converts a [`serde_json::Value`] to a [`Reflect`] value.
    ///
    /// Use `expected_type_path` to denote what type you might be expecting in case of ambiguity.
    /// For example, whether to return an [`i64`] or whether to downcast it to an [`i32`].
    ///
    /// Returns the result regardless or success or failure, but how you handle that is up to you.
    ///
    /// If you don't use an `expected_type_path`, then it should always return [`Ok`]. Feel free to `unwrap()` then if so
    fn to_reflect(
        &self,
        expected_type_path: Option<&str>,
    ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>>;
}

/// A string based data type that stores useful data to convert [`Scene`]'s and bevy_ecs [`Entity`]'s to strings and back.
#[derive(Debug)]
pub struct SerializedSceneData {
    pub name: String,
    pub entity_data: DataHashmap,
}

impl ToReflect for serde_json::Value {
    fn to_reflect(
        &self,
        expected_type: Option<&str>,
    ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
        let value = match self {
            Value::Null => Reflect::reflect_owned(Box::new(())),
            Value::Bool(bool) => Reflect::reflect_owned(Box::new(bool.to_owned())),
            Value::Number(number) => {
                let value: Box<dyn Reflect> = if let Some(int) = number.as_i64() {
                    if let None = expected_type {
                        Box::new(int)
                    } else {
                    }
                } else {
                    Box::new(number.as_f64().unwrap_or(0.0))
                };
                Reflect::reflect_owned(value)
            }
            Value::String(string) => Reflect::reflect_owned(Box::new(string.to_owned())),
            Value::Array(array) => todo!(),
            Value::Object(object) => todo!(),
        };
        Ok(Box::<dyn Reflect>::from(match value {
            bevy_reflect::ReflectOwned::Struct(e) => todo!(),
            bevy_reflect::ReflectOwned::TupleStruct(_) => todo!(),
            bevy_reflect::ReflectOwned::Tuple(_) => todo!(),
            bevy_reflect::ReflectOwned::List(_) => todo!(),
            bevy_reflect::ReflectOwned::Array(_) => todo!(),
            bevy_reflect::ReflectOwned::Map(_) => todo!(),
            bevy_reflect::ReflectOwned::Enum(en) => {
                dbg!(en.reflect_type_path());
                todo!()
            }
            bevy_reflect::ReflectOwned::Value(e) => e,
        }))
    }
}

impl SerializedSceneData {
    pub fn initialize(
        self,
        world: &mut World,
        type_registry: &TypeRegistry,
    ) -> serde_json::Result<component::Scene> {
        let scene = component::Scene::new(self.name.to_owned());

        for (entity_name, entity_hashmap) in self.entity_data {
            let bundle = SceneData {
                object_name: entity_name,
                scene_id: 0,
            };

            let mut entity = world.spawn(bundle);

            for (component_path, component_data) in entity_hashmap {
                let component_registration: &bevy_reflect::TypeRegistration =
                    type_registry.get_with_type_path(&component_path).unwrap();

                let fields = match component_registration.type_info() {
                    bevy_reflect::TypeInfo::Struct(struct_info) => struct_info.field_names(),
                    bevy_reflect::TypeInfo::TupleStruct(_) => todo!(), // These `todo!()` 's shouldn't be hit, but if they are, implement something here.
                    bevy_reflect::TypeInfo::Tuple(_) => todo!(),
                    bevy_reflect::TypeInfo::List(_) => todo!(),
                    bevy_reflect::TypeInfo::Array(_) => todo!(),
                    bevy_reflect::TypeInfo::Map(_) => todo!(),
                    bevy_reflect::TypeInfo::Enum(_) => todo!(),
                    bevy_reflect::TypeInfo::Value(_) => todo!(),
                };

                let mut component_patch = DynamicStruct::default();

                component_patch.set_represented_type(Some(component_registration.type_info()));

                let reflect_component = component_registration.data::<ReflectComponent>().unwrap();

                for (name, field) in component_data {
                    let expected_type = component_patch.field(&name).unwrap().reflect_type_path();

                    let value = match field.to_reflect(Some(expected_type)) {
                        Ok(ok) => ok,
                        Err(ok) => ok,
                    };

                    component_patch.insert_boxed(&name, value);
                }

                reflect_component.apply_or_insert(&mut entity, &dbg!(component_patch));
            }
        }

        Ok(scene)
    }
}

impl Serialize for SerializedSceneData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut serialize_struct = serializer.serialize_struct("SerializedScene", 2)?;

        serialize_struct.serialize_field("name", &self.name);
        serialize_struct.serialize_field("entity_data", &self.entity_data);
        serialize_struct.end()
    }
}

impl<'de> Deserialize<'de> for SerializedSceneData {
    /// Takes in a [`SerializedScene`] struct made from [`Scene`]`::serialize()` and turns it back into a scene
    ///
    /// Call [`SerializedScene`]::initialize() to turn it back into a useable [`Scene`]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_struct("SerializedScene", &["name", "entity_data"], SceneVisitor)
    }
}

/// Simple [`Visitor`] for deserializing [`SerializedScene`]'s from `Jesoon` (or whatever serializer is used) into a proper Rust data type
pub(crate) struct SceneVisitor;

impl<'de> Visitor<'de> for SceneVisitor {
    type Value = SerializedSceneData;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a serialized scene with an entities field")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut serialized_scene = SerializedSceneData {
            name: String::new(),
            entity_data: HashMap::new(),
        };

        // This at one point was like 20 lines long
        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "name" => serialized_scene.name = map.next_value()?,
                "entity_data" => serialized_scene.entity_data = map.next_value()?,
                _ => (),
            };
        }

        Ok(serialized_scene)
    }
}
