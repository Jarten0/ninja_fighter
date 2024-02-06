use bevy_ecs::{
    component::{Component, ComponentDescriptor, ComponentId, Tick},
    entity::Entity,
    query::{FilteredAccess, ReadOnlyWorldQuery, WorldQuery},
    storage::TableRow,
    world::unsafe_world_cell::UnsafeWorldCell,
};
use bevy_trait_query::{All, WriteTraits};
use erased_serde::{Error, Serializer};
use std::{alloc::Layout, borrow::Cow, path::PathBuf};

/// UGHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHH
/// so not excited to do this
///
/// A trait for implementing [`Scene`] serialization behaviour for your component
///
pub trait SerializableComponent: erased_serde::Serialize
where
    Self: Component<Storage = bevy_ecs::component::TableStorage>,
{
    /// Returns the path of where the component will be saved and loaded from.
    fn path(&self) -> PathBuf;

    fn descriptor() -> ComponentDescriptor
    where
        Self: Sized,
    {
        ComponentDescriptor::new::<Self>()
    }
}
erased_serde::serialize_trait_object!(SerializableComponent);

/// Holds data for the assigned [`Scene`] to operate upon.
/// An entity cannot be serialized by the [`Scene`] if it does not have this component.
///
// TODO: finish [`SceneData`] docs
#[derive(Component)]
pub struct SceneData {
    pub object_name: String,
    /// The ID of the current scene that the component holder belongs to.
    /// Not to be confused with the [`SceneObjectID`], which is a seperate thing uhh
    // TODO: figure that out
    pub scene_id: usize,
}

#[bevy_trait_query::queryable]
pub trait TestSuperTrait {
    fn erased_serialize(&self, serializer: &mut dyn Serializer) -> Result<(), Error>;
}

impl<T: erased_serde::Serialize + 'static> TestSuperTrait for T {
    fn erased_serialize(&self, serializer: &mut dyn Serializer) -> Result<(), erased_serde::Error> {
        <T as erased_serde::Serialize>::erased_serialize(self, serializer)
    }
}
