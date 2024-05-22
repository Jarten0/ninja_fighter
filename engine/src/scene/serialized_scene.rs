use crate::assets::SerializableAsset;
use crate::assets::SerializedAsset;
use crate::scene::object_id::ComponentInstanceID;
use crate::scene::IDCounter;

use super::add_entity_to_scene;
use super::component;
use super::object_data::SceneData;
use super::SceneError;
use bevy_ecs::entity::Entity;
use bevy_ecs::reflect::ReflectComponent;
use bevy_ecs::world::World;
use bevy_reflect::DynamicList;
use bevy_reflect::DynamicMap;
use bevy_reflect::DynamicStruct;
use bevy_reflect::DynamicTuple;
use bevy_reflect::DynamicTupleStruct;
use bevy_reflect::List;
use bevy_reflect::Reflect;
use bevy_reflect::ReflectDeserialize;
use bevy_reflect::ReflectOwned;
use bevy_reflect::Struct;
use bevy_reflect::TypeInfo;
use bevy_reflect::TypePath;
use bevy_reflect::TypeRegistration;
use bevy_reflect::TypeRegistry;
use log::*;
use serde::de::Visitor;
use serde::ser::SerializeStruct;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

pub type DataHashmap = HashMap<String, EntityHashmap>; // string = entity name, entity hashmap = entities owned components
pub type EntityHashmap = HashMap<String, ComponentData>; // static str = type path of component
pub type ComponentData = serde_json::Map<String, serde_json::Value>; // component data

/// Private trait that converts [`serde_json::Value`] to [`Reflect`]
pub trait ToReflect {
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
        type_registry: &TypeRegistry,
    ) -> Box<dyn Reflect>;
}

/// A string based data type that stores useful data to convert [`Scene`]'s and bevy_ecs [`Entity`]'s to strings and back.
#[derive(Debug)]
pub struct SerializedSceneData {
    pub name: String,
    pub entity_data: HashMap<String, HashMap<String, String>>,
    pub asset_data: HashMap<String, serde_json::Value>,
}

impl SerializedSceneData {
    /// Turns the [`SerializedSceneData`] into a [`Scene`], initializing every component and entity, and putting them into the world.
    ///
    /// Can throw a [`SceneError`] if no type registry for a type is found. Make sure to call `type_registry.register::<T>()` on your types.
    ///
    /// NOTE: If it doesn't state that your component was serialized, its because the functionality likely has yet to be implemented for the data structure type you're using.
    pub fn initialize(
        self,
        world: &mut World,
        type_registry: &TypeRegistry,
    ) -> Result<Entity, SceneError> {
        trace!("Initializing new scene ({})...", self.name);

        let scene = component::Scene::new(self.name.to_owned());

        let mut entities: Vec<Entity> = Vec::new();

        for (entity_name, component_data_hashmap) in self.entity_data {
            trace!("Initializing new entity ({})", &entity_name);
            trace!("Component list: {:?}", component_data_hashmap);

            let mut component_paths = HashMap::new();
            let mut component_ids = HashMap::new();
            component_data_hashmap
                .keys()
                .inspect(|path| {
                    trace!("Inserted new component path [{}]", path);
                    let k = ComponentInstanceID::get_new();
                    component_paths.insert(k, (*path).to_owned());
                    component_ids.insert((*path).to_owned(), k);
                })
                .last();

            let bundle = SceneData {
                entity_name,
                scene_id: Some(scene.scene_id),
                component_paths,
                component_ids,
                hide_in_inspector: true,
            };

            let entity_name_debug = bundle.entity_name.clone();

            let mut entity = world.spawn(bundle);

            trace!("Spawned {} with SceneData component", entity_name_debug);

            for (component_path, component_data) in component_data_hashmap {
                trace!("Initializing component {}", component_path);

                let component_registration: &bevy_reflect::TypeRegistration = type_registry
                    .get_with_type_path(&component_path)
                    .ok_or(SceneError::MissingTypeRegistry(component_path.to_owned()))?;

                let reflect_component = component_registration.data::<ReflectComponent>().ok_or(
                    SceneError::MissingTypeRegistry(format!(
                        "The {} component is missing a #[reflect(Component)] helper",
                        component_path
                    )),
                )?;

                let reflect_deserialize = component_registration
                    .data::<ReflectDeserialize>()
                    .ok_or(SceneError::NoSerializationImplementation(format!(
                    "The {} component is missing deserialization type data in the type registry",
                    component_path
                )))?;

                let type_info = component_registration.type_info();

                let mut deserializer = serde_json::Deserializer::from_str(component_data.as_str());
                let value = reflect_deserialize.deserialize(&mut deserializer).unwrap();
                deserializer.end().unwrap();
            }
            entities.push(entity.id());
        }

        for (asset_name, serialized_asset_data) in self.asset_data {
            trace!("Skipped {}. {:?}", asset_name, serialized_asset_data);
        }

        let scene_entity = world.spawn(scene).id();

        // We have to wait until after the scene entity is spawned before we can start adding entities to the scene component
        for entity in entities {
            let _ = add_entity_to_scene(world, scene_entity, entity, None);
        }
        Ok(scene_entity)
    }
}

impl Serialize for SerializedSceneData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut serialize_struct = serializer.serialize_struct("SerializedScene", 2)?;

        let _ = serialize_struct.serialize_field("name", &self.name);
        let _ = serialize_struct.serialize_field("entity_data", &self.entity_data);
        let _ = serialize_struct.serialize_field("asset_data", &self.asset_data);
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
        deserializer.deserialize_struct(
            "SerializedScene",
            &["name", "entity_data", "asset_data"],
            SceneVisitor,
        )
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
            asset_data: HashMap::new(),
        };

        // This at one point was like 20 lines long
        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "name" => serialized_scene.name = map.next_value()?,
                "entity_data" => serialized_scene.entity_data = map.next_value()?,
                "asset_data" => serialized_scene.asset_data = map.next_value()?,
                _ => (),
            };
        }

        Ok(serialized_scene)
    }
}

//

// TODO: Pull this out and move it to it's own module in [`scene`]
impl ToReflect for serde_json::Value {
    fn to_reflect(
        &self,
        expected_type: Option<&str>,
        type_registry: &TypeRegistry,
    ) -> Box<dyn Reflect> {
        let type_info = type_registry
            .get_with_type_path(expected_type.unwrap())
            .unwrap();

        let binding = self.to_string();
        let mut deserializer = serde_json::Deserializer::from_str(binding.as_str());
        let deserialize = type_info
            .data::<ReflectDeserialize>()
            .unwrap()
            .deserialize(&mut deserializer)
            .unwrap();
        return deserialize;

        let value = match self {
            Value::Null => Reflect::reflect_owned(Box::new(())),
            Value::Bool(bool) => Reflect::reflect_owned(Box::new(bool.to_owned())),
            Value::Number(number) => convert_number(number, expected_type),
            Value::String(string) => convert_string(string, expected_type),
            Value::Array(array) => convert_array(array, expected_type, type_registry),
            Value::Object(object) => convert_struct(object, expected_type, type_registry),
        };
        Box::<dyn Reflect>::from(match value {
            bevy_reflect::ReflectOwned::Struct(e) => e.into_reflect(),
            bevy_reflect::ReflectOwned::TupleStruct(ts) => ts.into_reflect(),
            bevy_reflect::ReflectOwned::Tuple(t) => t.into_reflect(),
            bevy_reflect::ReflectOwned::List(l) => l.into_reflect(),
            bevy_reflect::ReflectOwned::Array(a) => a.into_reflect(),
            bevy_reflect::ReflectOwned::Map(m) => m.into_reflect(),
            bevy_reflect::ReflectOwned::Enum(en) => en.into_reflect(),
            bevy_reflect::ReflectOwned::Value(e) => e,
        })
    }
}

fn convert_string(string: &String, expected_type: Option<&str>) -> bevy_reflect::ReflectOwned {
    if expected_type == Some(char::type_path()) {
        return Reflect::reflect_owned(Box::new(*string.as_bytes().get(0).unwrap() as char));
    }

    Reflect::reflect_owned(Box::new(string.to_owned()))
}

fn convert_number(
    number: &serde_json::Number,
    expected_type: Option<&str>,
) -> bevy_reflect::ReflectOwned {
    let value: Box<dyn Reflect> = if let Some(int) = number.as_i64() {
        downcast_int(expected_type, int)
    } else {
        let x = number.as_f64().unwrap_or(0.0);

        if expected_type == Some(f32::type_path()) {
            downcast_float(x)
        } else {
            Box::new(x)
        }
    };
    Reflect::reflect_owned(value)
}

fn convert_array(
    jesoon_array: &Vec<Value>,
    expected_type: Option<&str>,
    type_registry: &TypeRegistry,
) -> bevy_reflect::ReflectOwned {
    if let Some(expected_type) = expected_type {
        if let Some(type_registration) = type_registry.get_with_type_path(expected_type) {
            let owned = match type_registration.type_info() {
                TypeInfo::Struct(_) => todo!(),
                TypeInfo::TupleStruct(tsinfo) => {
                    let mut dyn_ts = DynamicTupleStruct::default();

                    dyn_ts.set_represented_type(Some(type_registration.type_info()));

                    for i in 0..tsinfo.field_len() {
                        let field = tsinfo.field_at(i).unwrap();
                        let value = jesoon_array.get(i).unwrap();
                        dyn_ts
                            .insert_boxed(value.to_reflect(Some(field.type_path()), type_registry));
                    }
                    ReflectOwned::TupleStruct(Box::new(dyn_ts))
                }
                TypeInfo::Tuple(_) => todo!(),
                TypeInfo::List(_) => todo!(),
                TypeInfo::Array(_) => todo!(),
                TypeInfo::Map(_) => todo!(),
                TypeInfo::Enum(_) => todo!(),
                TypeInfo::Value(_) => todo!(),
            };

            trace!("Converted JSON array into ReflectOwned");

            owned
        } else {
            if let Some(generic_type) = expected_type.strip_prefix("alloc::vec::Vec") {
                let vec_type = generic_type.trim_start_matches('<').trim_end_matches('>');
                let reflect_values = jesoon_array
                    .iter()
                    .map(|value| value.to_reflect(Some(vec_type), type_registry))
                    .collect::<Vec<Box<dyn Reflect>>>();

                let mut dyn_list = DynamicList::default();

                for item in reflect_values {
                    dyn_list.insert(dyn_list.len(), item);
                }

                return bevy_reflect::ReflectOwned::List(Box::new(dyn_list));
            }

            trace!("No type registration found for {}", expected_type);

            todo!()
        }
    } else {
        trace!("No expected type for array conversion");

        todo!()
    }
}

#[allow(unused)]
fn convert_struct(
    jesoon_object: &serde_json::Map<String, Value>,
    expected_type: Option<&str>,
    type_registry: &TypeRegistry,
) -> ReflectOwned {
    let type_path = expected_type.unwrap();

    let type_info = type_registry
        .get_with_type_path(type_path)
        .expect(&format!(
            "Expected a registered value type, got {}",
            type_path
        ))
        .type_info();

    if let TypeInfo::Struct(s_info) = type_info {
        let mut dyn_struct = DynamicStruct::default();

        for (name, field) in jesoon_object {
            if let Some(some) = s_info.field(name) {
                let expected_type_path = Some(some.type_path());

                dyn_struct.insert_boxed(name, field.to_reflect(expected_type_path, type_registry));
            } else {
                dyn_struct.insert_boxed(name, field.to_reflect(None, type_registry));
            }
        }

        return ReflectOwned::Struct(Box::new(dyn_struct));
    }
    if let TypeInfo::TupleStruct(ts_info) = type_info {
        todo!()
    }
    if let TypeInfo::Tuple(t_info) = type_info {
        todo!()
    }
    if let TypeInfo::List(l_info) = type_info {
        todo!()
    }
    if let TypeInfo::Array(a_info) = type_info {
        todo!()
    }
    if let TypeInfo::Map(m_info) = type_info {
        let dyn_map = DynamicMap::default();

        // let key_expected_type = expected_type
        // let field_expected_type
        for (name, field) in jesoon_object {
            let type_info = type_registry.get_with_type_path(todo!()).unwrap();

            let reflect_deserialize = type_info.data::<ReflectDeserialize>().unwrap();

            // reflect_deserialize.deserialize(serde_json::Deserializer::from_str(s));

            log::trace!("{}   {}: {}", expected_type.unwrap(), name, field);
            std::thread::sleep(std::time::Duration::from_millis(10));
        }

        todo!()
    }
    if let TypeInfo::Enum(e_info) = type_info {
        todo!()
    }
    if let TypeInfo::Value(v_info) = type_info {
        todo!()
    }

    unreachable!()
}

fn convert_newtype_tuple_struct(
    jesoon_object: &serde_json::Map<String, Value>,
    expected_type: Option<&str>,
    type_registry: &TypeRegistry,
) -> bevy_reflect::ReflectOwned {
    if expected_type.is_none() {
        todo!();
    }

    let TypeInfo::TupleStruct(tuple_struct_type_info) = type_registry
        .get_with_type_path(expected_type.unwrap())
        .expect("todo")
        .type_info()
    else {
        panic!()
    };

    let field_path = tuple_struct_type_info.field_at(0).unwrap().type_path();
    let inner_struct_type_info = type_registry.get_with_type_path(field_path).unwrap();

    match inner_struct_type_info.type_info() {
        TypeInfo::Struct(s) => {
            let mut dynamic_struct = DynamicStruct::default();

            for field in s.iter() {
                dynamic_struct.insert_boxed(
                    field.name(),
                    jesoon_object
                        .get(field.name())
                        .unwrap()
                        .to_reflect(Some(field.type_path()), type_registry),
                );
            }

            return bevy_reflect::ReflectOwned::Struct(Box::new(dynamic_struct));
        }
        TypeInfo::TupleStruct(ts) => {
            let mut dynamic_tuple_struct = DynamicTupleStruct::default();

            for field in ts.iter() {
                dynamic_tuple_struct.insert_boxed(
                    jesoon_object
                        .get(&field.index().to_string())
                        .unwrap()
                        .to_reflect(Some(field.type_path()), type_registry),
                )
            }

            return ReflectOwned::TupleStruct(Box::new(dynamic_tuple_struct));
        }
        TypeInfo::Tuple(t) => {
            let mut dynamic_tuple = DynamicTuple::default();

            for field in t.iter() {
                dynamic_tuple.insert_boxed(
                    jesoon_object
                        .get(field.index().to_string().as_str())
                        .unwrap()
                        .to_reflect(Some(field.type_path()), type_registry),
                )
            }

            return ReflectOwned::Tuple(Box::new(dynamic_tuple));
        }
        TypeInfo::List(l) => todo!(),
        TypeInfo::Array(a) => todo!(),
        TypeInfo::Map(m) => todo!(),
        TypeInfo::Enum(e) => todo!(),
        TypeInfo::Value(v) => {
            return bevy_reflect::ReflectOwned::Value(
                jesoon_object
                    .get("0")
                    .unwrap()
                    .to_reflect(Some(field_path), type_registry),
            );
        }
    }

    todo!()
}

/// Downcasts an int to the expected type
fn downcast_int(expected_type: Option<&str>, int: i64) -> Box<dyn Reflect> {
    match expected_type {
        None => Box::new(int),
        Some(expected_type) => {
            if expected_type == i32::type_path() {
                Box::new(int as i32)
            } else if expected_type == i16::type_path() {
                Box::new(int as i16)
            } else if expected_type == i8::type_path() {
                Box::new(int as i8)
            } else if expected_type == u128::type_path() {
                Box::new(int as u128)
            } else if expected_type == u64::type_path() {
                Box::new(int as u64)
            } else if expected_type == u32::type_path() {
                Box::new(int as u32)
            } else if expected_type == u16::type_path() {
                Box::new(int as u16)
            } else if expected_type == u8::type_path() {
                Box::new(int as u8)
            } else {
                Box::new(int)
            }
        }
    }
}

/// Downcasts a float to an f32, if possible
fn downcast_float(float: f64) -> Box<dyn Reflect> {
    if float < (f32::MIN as f64) || float > (f32::MAX as f64) {
        Box::new(float)
    } else {
        Box::new(float as f32)
    }
}
