use std::collections::HashMap;

use bevy_ecs::{
    component::{Component, ComponentId},
    world::{FromWorld, World},
};
use bevy_reflect::{reflect_trait, Reflect};
use erased_serde::{Error, Serializer};
use serde::Serialize;

use super::{object_id::ComponentInstanceID, ObjectID};

/// Holds data for the assigned [`Scene`] to operate upon.
/// An entity cannot be serialized by the [`Scene`] if it does not have this component.
///
// TODO: finish [`SceneData`] docs
#[derive(Component, Reflect, Debug)]
pub struct SceneData {
    /// Describes the name of the entity that this component belongs to.
    ///
    /// Is used for serialization, so using this is quite important.
    pub object_name: String,
    /// The ID of the current scene that the component holder belongs to.
    pub scene_id: Option<ObjectID>,
    /// Contains the component path of every component that is reflectable.
    pub component_paths: HashMap<ComponentInstanceID, String>,
    pub component_ids: HashMap<String, ComponentInstanceID>,
    /// Can be enabled to prevent the entity from being shown in the inspector.
    pub hide_in_inspector: bool,
}

impl Serialize for SceneData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str("Placeholder for scenedata")
    }
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
