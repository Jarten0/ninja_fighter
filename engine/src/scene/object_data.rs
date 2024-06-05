use std::{any::Any, collections::HashMap};

use bevy_ecs::{
    component::{Component, ComponentId},
    system::{Query, Res},
    world::{FromWorld, World},
};
use bevy_reflect::{reflect_trait, FromType, Reflect};
use erased_serde::{Error, Serializer};
use serde::Serialize;

use super::{object_id::ComponentInstanceID, ObjectID, Scene, SceneManager};

/// Holds data from the assigned [`Scene`] to operate upon.
/// An entity cannot be serialized by the [`Scene`] if it does not have this component.
///
/// To get a SceneData component, simply register your component with the [`SceneManager`]
// TODO: finish [`SceneData`] docs
#[derive(Component, Reflect, Debug)]
pub struct SceneData {
    /// Describes the name of the entity that this component belongs to.
    ///
    /// Is used for serialization, so using this is quite important.
    pub entity_name: String,
    /// The ID of the current scene that the component holder belongs to.
    pub scene_id: Option<ObjectID>,
    /// Contains the component path of every component that is reflectable.
    // pub component_paths: HashMap<ComponentInstanceID, String>,
    // pub component_ids: HashMap<String, ComponentInstanceID>,
    /// Can be enabled to prevent the entity from being shown in the inspector.
    pub hide_in_inspector: bool,
}

/// A trait for serializing components, must be implemented to serialize and deserialize components.
///
/// To implement, simply importing it should do.
///
/// However, it does require that the component implements Component, Reflect, Serialize, and is 'static
#[bevy_trait_query::queryable]
#[reflect_trait]
pub trait TestSuperTrait
where
    Self: 'static,
{
    fn as_reflect(&self) -> &dyn Reflect;

    /// Return a bool determining if the component should be visible in the inspector or not.
    ///
    /// Defaults to `true`.
    fn show_in_inspector(&self) -> bool {
        true
    }
}

impl<T> TestSuperTrait for T
where
    T: Component + Reflect + Default + FromWorld,
{
    fn as_reflect(&self) -> &dyn Reflect
    where
        Self: Reflect,
    {
        <Self as Reflect>::as_reflect(self)
    }
}

pub trait CustomSerialization {
    fn serialize_data(
        type_data: &CustomSerializationData,
        value: &dyn Reflect,
    ) -> serde_json::Map<String, serde_json::Value>;
}

#[derive(Clone)]
pub struct CustomSerializationData {
    serialize_data_fn: fn(&Self, &dyn Reflect) -> serde_json::Map<String, serde_json::Value>,
}

impl CustomSerializationData {
    pub(crate) fn serialize_data(
        &self,
        value: &dyn Reflect,
    ) -> serde_json::Map<String, serde_json::Value> {
        (self.serialize_data_fn)(self, value)
    }
}

impl<T: CustomSerialization> FromType<T> for CustomSerializationData {
    fn from_type() -> Self {
        Self {
            serialize_data_fn: <T as CustomSerialization>::serialize_data,
        }
    }
}
