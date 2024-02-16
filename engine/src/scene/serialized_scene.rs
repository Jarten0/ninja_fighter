use crate::scene::resource::SceneManager;

use super::component;
use super::traits::SceneData;
use bevy_ecs::component::ComponentId;
use bevy_ecs::reflect::ReflectComponent;
use bevy_ecs::schedule::DynEq;
use bevy_ecs::world::Ref;
use bevy_ecs::world::World;
use bevy_reflect::serde::UntypedReflectDeserializer;
use bevy_reflect::DynamicStruct;
use bevy_reflect::Enum;
use bevy_reflect::Reflect;
use bevy_reflect::TypeData;
use bevy_reflect::TypeRegistry;
use core::panic;
use serde::de::DeserializeSeed;
use serde::de::Visitor;
use serde::ser::SerializeStruct;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde_json::json;
use serde_json::Value;
use std::any::Any;
use std::any::TypeId;
use std::collections::HashMap;
use std::ops::Deref;

pub type DataHashmap = HashMap<String, EntityHashmap>;
pub type EntityHashmap = HashMap<String, ComponentHashmap>;
pub type ComponentHashmap = HashMap<String, Value>;

/// A string based data type that stores useful data to convert [`Scene`]'s and bevy_ecs [`Entity`]'s to strings and back.
#[derive(Debug)]
pub struct SerializedSceneData {
    pub name: String,
    pub entity_data: DataHashmap,
}

trait ToReflect {
    fn to_reflect(&self) -> Box<dyn Reflect>;
}

impl ToReflect for serde_json::Value {
    fn to_reflect(&self) -> Box<dyn Reflect> {
        match self {
            Value::Null => todo!(),
            Value::Bool(_) => todo!(),
            Value::Number(_) => todo!(),
            Value::String(_) => todo!(),
            Value::Array(_) => todo!(),
            Value::Object(_) => todo!(),
        }
    }
}

trait Clonable {
    fn clon(&self) -> Self;
}

impl Clonable for TypeRegistry {
    fn clon(&self) -> Self {
        // self.deref().clone()
    }
}

impl SerializedSceneData {
    pub fn initialize(self, world: &mut World) -> serde_json::Result<component::Scene> {
        let scene = component::Scene::new(self.name.to_owned());

        let registry = world.resource::<SceneManager>().registry;

        for (entity_name, entity_hashmap) in self.entity_data {
            let bundle = SceneData {
                object_name: entity_name,
                scene_id: 0,
            };
            let mut entity = world.spawn(bundle);
            for (component_path, component_data) in entity_hashmap {
                let component_registration: &bevy_reflect::TypeRegistration =
                    registry.get_with_type_path(&component_path).unwrap();

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

                let reflect_component = component_registration.data::<ReflectComponent>().unwrap();

                for (name, field) in component_data {
                    component_patch.insert_boxed(&name, field.to_reflect());
                }

                reflect_component.apply_or_insert(&mut entity, &component_patch);
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
