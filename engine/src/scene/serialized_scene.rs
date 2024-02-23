use super::add_entity_to_scene;
use super::component;
use super::traits::SceneData;
use super::SceneError;
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::reflect::ReflectComponent;
use bevy_ecs::world::World;
use bevy_reflect::DynamicStruct;
use bevy_reflect::Reflect;
use bevy_reflect::Struct;
use bevy_reflect::TypeData;
use bevy_reflect::TypePath;
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
            Value::Number(number) => convert_number(number, expected_type),
            Value::String(string) => convert_string(string, expected_type),
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
                en.reflect_type_path();
                todo!()
            }
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
        let scene = component::Scene::new(self.name.to_owned());

        let mut entities: Vec<Entity> = Vec::new();
        for (entity_name, entity_hashmap) in self.entity_data {
            let bundle = SceneData {
                object_name: entity_name,
                scene_id: 0,
            };

            let mut entity = world.spawn(bundle);

            for (component_path, component_data) in entity_hashmap {
                let component_registration: &bevy_reflect::TypeRegistration = type_registry
                    .get_with_type_path(&component_path)
                    .ok_or(SceneError::MissingTypeRegistry(component_path.clone()))?;

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

                for (name, field) in &component_data {
                    println!("!");
                    let f = || {
                        panic!(
                            "No expected type found! tried to find the field named [{:?}] on {:?}. ",
                            name, component_path
                        )
                    };

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

                    let value = match field.to_reflect(Some(expected_type)) {
                        Ok(ok) => ok,
                        Err(ok) => ok,
                    };

                    component_patch.insert_boxed(&name, value);
                }
                let reflect_component = component_registration.data::<ReflectComponent>().unwrap();

                reflect_component.apply_or_insert(&mut entity, &component_patch);
            }

            entities.push(entity.id());
        }

        let scene_entity = world.spawn(scene).id();

        // We have to wait until after the scene entity is spawned before we can start adding entities to the scene component
        for entity in entities {
            add_entity_to_scene(world, scene_entity, entity);
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
