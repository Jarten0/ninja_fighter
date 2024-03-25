use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use bevy_ecs::{
    component::{Component, ComponentId, ComponentStorage},
    entity::Entity,
    query::Added,
    storage::Table,
    system::Query,
    world::World,
};
use bevy_reflect::{reflect_trait, Reflect, TypePath};
use erased_serde::{Error, Serializer};
use serde::Serialize;

use super::ObjectID;

/// Holds data for the assigned [`Scene`] to operate upon.
/// An entity cannot be serialized by the [`Scene`] if it does not have this component.
///
// TODO: finish [`SceneData`] docs
#[derive(Component, Reflect)]
pub struct SceneData {
    /// Describes the name of the entity that this component belongs to.
    ///
    /// Is used for serialization, so using this is quite important.
    pub object_name: String,
    /// The ID of the current scene that the component holder belongs to.
    pub scene_id: Option<ObjectID>,
}

impl Serialize for SceneData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str("Placeholdre for scenedata")
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
    fn erased_serialize(&self, serializer: &mut dyn Serializer) -> Result<(), Error>;

    fn get_component_id(world: &World) -> Option<ComponentId>
    where
        Self: Sized + Component;

    fn component_id(world: &World) -> ComponentId
    where
        Self: Sized + Component;

    fn as_reflect(&self) -> &dyn Reflect;
}

impl<T> TestSuperTrait for T
where
    T: erased_serde::Serialize + Component + Reflect + serde::Serialize,
{
    fn erased_serialize(&self, serializer: &mut dyn Serializer) -> Result<(), erased_serde::Error> {
        <T as erased_serde::Serialize>::erased_serialize(self, serializer)
    }

    /// Gets the current [`ComponentId`] for the object
    fn get_component_id(world: &World) -> Option<ComponentId>
    where
        Self: Sized + Component,
    {
        world.component_id::<Self>()
    }

    /// Get the current [`ComponentId`] of the object.
    ///
    /// Panicking version of [`TestSuperTrait::get_component_id`], fails when the component has yet to be initialized in the world.
    fn component_id(world: &World) -> ComponentId
    where
        Self: Sized + Component,
    {
        world.component_id::<Self>().unwrap()
    }

    fn as_reflect(&self) -> &dyn Reflect
    where
        Self: Reflect,
    {
        <Self as Reflect>::as_reflect(self)
    }
}
