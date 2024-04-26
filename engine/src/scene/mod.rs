mod component;
mod converter;
mod error;
mod object_data;
mod object_id;
mod scene_manager;
mod serialized_scene;
#[cfg(test)]
mod test;

use std::any::TypeId;

use bevy_ecs::reflect::ReflectFromWorld;
use bevy_ecs::world::FromWorld;
use bevy_ecs::{component::Component, world::Mut};
use bevy_reflect::{FromReflect, GetTypeRegistration, Reflect, TypePath};
use bevy_trait_query::RegisterExt;

use crate::editor::{FieldWidget, InspectableAsField};
use crate::space::{self, Vector2};
pub use component::Scene;
pub use component::{
    add_entity_to_scene, load_scene, new_scene, save_scene, to_serializable_scene_data,
    unload_scene, validate_name,
};
pub use converter::*;
pub use error::SceneError;
pub use object_data::ReflectTestSuperTrait;
pub use object_data::SceneData;
pub use object_data::TestSuperTrait;
pub use object_id::CounterType;
pub use object_id::ObjectID;
pub use object_id::{ComponentInstanceID, Counter, IDCounter};
pub use scene_manager::SceneManager;
pub use serialized_scene::ToReflect;
