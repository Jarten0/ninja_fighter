use super::add_entity_to_scene;
use super::component;
use super::object_data::SceneData;
use super::SceneError;
use bevy_ecs::entity::Entity;
use bevy_ecs::reflect::ReflectComponent;
use bevy_ecs::world::World;
use bevy_reflect::DynamicList;
use bevy_reflect::DynamicStruct;
use bevy_reflect::DynamicTupleStruct;
use bevy_reflect::List;
use bevy_reflect::Reflect;
use bevy_reflect::ReflectOwned;
use bevy_reflect::TypePath;
use bevy_reflect::TypeRegistry;
use log::*;
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
        type_registry: &TypeRegistry,
    ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
        let value = match self {
            Value::Null => Reflect::reflect_owned(Box::new(())),
            Value::Bool(bool) => Reflect::reflect_owned(Box::new(bool.to_owned())),
            Value::Number(number) => convert_number(number, expected_type),
            Value::String(string) => convert_string(string, expected_type),
            Value::Array(array) => convert_array(array, expected_type, type_registry),
            Value::Object(_object) => {
                dbg!(expected_type);
                todo!()
            }
        };
        Ok(Box::<dyn Reflect>::from(match value {
            bevy_reflect::ReflectOwned::Struct(e) => e.into_reflect(),
            bevy_reflect::ReflectOwned::TupleStruct(ts) => ts.into_reflect(),
            bevy_reflect::ReflectOwned::Tuple(t) => t.into_reflect(),
            bevy_reflect::ReflectOwned::List(l) => l.into_reflect(),
            bevy_reflect::ReflectOwned::Array(a) => a.into_reflect(),
            bevy_reflect::ReflectOwned::Map(m) => m.into_reflect(),
            bevy_reflect::ReflectOwned::Enum(en) => en.into_reflect(),
            bevy_reflect::ReflectOwned::Value(e) => e,
        }))
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
                bevy_reflect::TypeInfo::Struct(_) => todo!(),
                bevy_reflect::TypeInfo::TupleStruct(tsinfo) => {
                    let mut dyn_ts = DynamicTupleStruct::default();

                    dyn_ts.set_represented_type(Some(type_registration.type_info()));

                    for i in 0..tsinfo.field_len() {
                        let field = tsinfo.field_at(i).unwrap();
                        let value = jesoon_array.get(i).unwrap();
                        dyn_ts.insert_boxed(
                            value
                                .to_reflect(Some(field.type_path()), type_registry)
                                .unwrap(),
                        );
                    }
                    ReflectOwned::TupleStruct(Box::new(dyn_ts))
                }
                bevy_reflect::TypeInfo::Tuple(_) => todo!(),
                bevy_reflect::TypeInfo::List(_) => todo!(),
                bevy_reflect::TypeInfo::Array(_) => todo!(),
                bevy_reflect::TypeInfo::Map(_) => todo!(),
                bevy_reflect::TypeInfo::Enum(_) => todo!(),
                bevy_reflect::TypeInfo::Value(_) => todo!(),
            };

            trace!("Converted JSON array into ReflectOwned");

            owned
        } else {
            if let Some(generic_type) = expected_type.strip_prefix("alloc::vec::Vec") {
                let vec_type = generic_type.trim_start_matches('<').trim_end_matches('>');
                let reflect_values = jesoon_array
                    .iter()
                    .map(|value| value.to_reflect(Some(vec_type), type_registry).unwrap())
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

impl SerializedSceneData {
    /// Turns the [`SerializedSceneData`] into a [`Scene`], initializing every component and entity, and putting them into the world.
    ///
    /// Can throw a [`SceneError`] if no type registry for a type is found. Make sure to call `type_registry.register::<T>()` on your types.
    pub fn initialize(
        self,
        world: &mut World,
        type_registry: &TypeRegistry,
    ) -> Result<Entity, SceneError> {
        trace!("Initializing new scene...");

        let scene = component::Scene::new(self.name.to_owned());

        let mut entities: Vec<Entity> = Vec::new();
        for (entity_name, entity_hashmap) in self.entity_data {
            trace!("Initializing {}", &entity_name);

            let bundle = SceneData {
                object_name: entity_name,
                scene_id: Some(scene.scene_id),
            };

            let entity_name_debug = bundle.object_name.clone();

            let mut entity = world.spawn(bundle);

            trace!(
                "Spawned {} entity with SceneData component",
                entity_name_debug
            );

            for (component_path, component_data) in entity_hashmap {
                trace!("Initializing component {}", component_path);

                let component_registration: &bevy_reflect::TypeRegistration = type_registry
                    .get_with_type_path(&component_path)
                    .ok_or(SceneError::MissingTypeRegistry(component_path.clone()))?;

                let mut component_patch = DynamicStruct::default();

                component_patch.set_represented_type(Some(component_registration.type_info()));

                for (name, field) in &component_data {
                    trace!("Initializing field {}: {}", field, name);

                    let type_info = match component_registration.type_info() {
                        bevy_reflect::TypeInfo::Struct(struct_info) => struct_info,
                        bevy_reflect::TypeInfo::TupleStruct(_) => todo!(),
                        bevy_reflect::TypeInfo::Tuple(_) => todo!(),
                        bevy_reflect::TypeInfo::List(_) => todo!(),
                        bevy_reflect::TypeInfo::Array(_) => todo!(),
                        bevy_reflect::TypeInfo::Map(_) => todo!(),
                        bevy_reflect::TypeInfo::Enum(_) => todo!(),
                        bevy_reflect::TypeInfo::Value(_) => todo!(),
                    };

                    let expected_type = type_info
                        .field(name)
                        .ok_or(SceneError::MissingTypeRegistry(component_path.clone()))?
                        .type_path();

                    let value = match field.to_reflect(Some(expected_type), type_registry) {
                        Ok(ok) => ok,
                        Err(ok) => {
                            error!(
                                "Could not conform {} to the expected type {}",
                                name, expected_type
                            );
                            ok
                        }
                    };

                    trace!("Initialized {}", name);

                    component_patch.insert_boxed(&name, value);
                }
                let reflect_component = component_registration.data::<ReflectComponent>().ok_or(
                    SceneError::MissingTypeRegistry(format!(
                        "The {} component is missing a #[reflect(Component)] helper",
                        component_path
                    )),
                )?;

                reflect_component.apply_or_insert(&mut entity, &component_patch);
            }

            entities.push(entity.id());
        }

        let scene_entity = world.spawn(scene).id();

        // We have to wait until after the scene entity is spawned before we can start adding entities to the scene component
        for entity in entities {
            let _ = add_entity_to_scene(world, scene_entity, entity);
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
