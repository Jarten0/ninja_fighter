use crate::scene::resource::SceneManager;

use super::component;
use bevy_ecs::reflect::ReflectComponent;
use bevy_ecs::world::Ref;
use bevy_ecs::world::World;
use bevy_reflect::serde::UntypedReflectDeserializer;
use bevy_reflect::DynamicStruct;
use bevy_reflect::Reflect;
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
use std::any::TypeId;
use std::collections::HashMap;

pub type DataHashmap = HashMap<String, EntityHashmap>;
pub type EntityHashmap = HashMap<String, ComponentHashmap>;
pub type ComponentHashmap = HashMap<String, Value>;

/// A string based data type that stores useful data to convert [`Scene`]'s and bevy_ecs [`Entity`]'s to strings and back.
#[derive(Debug)]
pub struct SerializedSceneData {
    pub name: String,
    pub entity_data: DataHashmap,
}

impl SerializedSceneData {
    pub fn initialize(self, world: &mut World) -> serde_json::Result<component::Scene> {
        let scene = component::Scene::new(self.name.to_owned());
        // TODO: Deserialize entity data here and add it to the scene

        let registry = &world.resource::<SceneManager>().registry;

        let reflect_deserializer = UntypedReflectDeserializer::new(&registry);

        for (entity_name, entity_hashmap) in self.entity_data {
            for (component_path, component_data) in entity_hashmap {
                let serialized_value = &component_data;
                let mut component_patch = DynamicStruct::default();

                let reflect_deserializer = UntypedReflectDeserializer::new(registry);

                for (name, field) in component_data {
                    let s = String::new();
                    let mut json = serde_json::Deserializer::from_str(&s);
                    let value = reflect_deserializer.deserialize(&mut json).unwrap();
                    component_patch.insert::<dyn Reflect>(&name, value.into());
                }
                // let type_data = type_registry_data.data().unwrap();

                let deserialized_value: Box<dyn Reflect> = todo!();
            }
        }

        // Convert
        // let converted_value =
        // <MyStruct as FromReflect>::from_reflect(&*deserialized_value).unwrap();

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
