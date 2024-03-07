use bevy_ecs::{
    component::{Component, ComponentId},
    world::World,
};
use bevy_reflect::Reflect;
use erased_serde::{Error, Serializer};

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
    ///
    /// Not to be confused with the [`SceneObjectID`], which is a seperate thing uhh
    // TODO: figure that out
    pub scene_id: usize,
}

#[bevy_trait_query::queryable]
pub trait TestSuperTrait {
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
    T: erased_serde::Serialize + 'static + Component + Reflect,
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
        self.as_reflect()
    }
}
