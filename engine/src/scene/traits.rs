use bevy_ecs::component::{Component, ComponentDescriptor, ComponentId};
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
    pub root_scene: String,
    /// The ID of the current scene that the component holder belongs to.
    /// Not to be confused with the [`SceneObjectID`], which is a seperate thing uhh
    // TODO: figure that out
    pub scene_id: usize,
    pub serializeable_components: Vec<ComponentId>,
}
