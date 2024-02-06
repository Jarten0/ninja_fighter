use crate::scene::serialize::SerializedSceneData;
use crate::scene::traits::SceneData;

use super::serialize;
use super::traits;
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::query::With;
use bevy_ecs::system::Commands;
use bevy_ecs::world::World;
use bevy_trait_query::All;
use bevy_trait_query::One;
use core::panic;
use erased_serde::Serializer;
use serde::Serialize;
use std;
use std::env::current_dir;
use std::fs::File;
use std::ops::Deref;
use std::path::PathBuf;

/// Entity managment for loading and unloading in batches rather than having everything loaded at once.
///
/// Each [`Entity`] must have the [`SceneData`] component if it wishes to be managed by a scene.
///
/// If you'd rather have your entities not managed by a scene, you can simply omit the [`SceneData`] component.
///
/// `entities_initialized`: if the scene has been deserialized but the entities have not been spawned, this will be false.
/// Otherwise, this should always be true.
///
/// The comparison operator (`==`) is supported, but because comparing entities requires a [`World`] to obtain component data, using it only compares the names.
/// If you want a true eq operation that checks if the entities match, use [`Scene`]`::entity_eq()`. This does require [`World`] access though.
#[derive(Debug, Component, Clone)]
pub struct Scene {
    /// The name of the scene
    ///
    /// Primarily used for serialization, but can also be handy for organization and managing active scenes.
    ///
    /// Ideally, no two scenes should share the same name. Handling that is the responsibility of the [`SceneManager`]
    pub name: String,
    /// Contains an [`Entity`] id for every entity that this [`Scene`] is responsible for.
    /// Be careful when manually adjusting which entities are stored,
    /// since every entity is required to own one [`SceneData`] component.
    ///
    /// Using the API to add or remove entities ensures that every entity has a [`SceneData`] component,
    /// and ensures that no entity is orphaned and never unloaded. (unless requested)
    pub(crate) entities: Vec<Entity>,
    /// The current stored save data.
    pub(crate) serialized_entity_component_data: Option<Vec<String>>,
    /// The path where the scene's save data is stored when calling [`save_scene`]
    pub(crate) save_data_path: PathBuf,
}

impl Scene {
    /// Creates a new blank [`Scene`] using the provided name.
    ///
    /// Does not contain any [`Entity`]'s and does not load any. To do that, wait until its ready.
    /// // TODO: When done, update these docs
    pub fn new(name: String) -> Self {
        let mut save_data_path = PathBuf::new();
        save_data_path.push(current_dir().unwrap());
        save_data_path.pop();
        save_data_path.push("test_output");
        save_data_path.push("test_save.json");
        Self {
            name,
            entities: Vec::new(),
            serialized_entity_component_data: None,
            save_data_path,
        }
    }

    /// The comparison operator (`==`) but with entity comparison.
    /// This checks every entity to see if their [`SceneData`] matches, and if not, returns `false`.
    ///
    /// If you don't want to compare [`Scene`]s with world access, you can use the default `==` operator implementation.
    /// However, it will only compare the scene's IDs, so entity checking is not viable.'/:`12`
    pub fn entity_eq(&self, other: &Scene, world: &World) -> bool {
        if self.name != other.name {
            return false;
        }

        for i in 0..self.entities.len() {
            let self_entity = if let Some(entity) = other.entities.get(i) {
                world
                    .entity(entity.to_owned())
                    .get::<traits::SceneData>()
                    .unwrap()
                    .scene_id
            } else {
                return false;
            };

            let other_entity = if let Some(entity) = self.entities.get(i) {
                world
                    .entity(entity.to_owned())
                    .get::<traits::SceneData>()
                    .unwrap()
                    .scene_id
            } else {
                return false;
            };

            if self_entity != other_entity {
                return false;
            }
        } // entities presumed to match after this point

        true
    }
}

impl PartialEq for Scene {
    /// Compares the names of the two scenes to check if they are the same scene.
    ///
    /// Incredibly fallible, but works reasonably within the confines of the [`Scene`] system
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

/// Serializes all of the entities with their components and stores it to a file (currently a temporary one)
pub fn save_scene(entity: Entity, world: &mut World) -> Result<(), std::io::Error> {
    let mut path = &world.get::<Scene>(entity).unwrap().save_data_path;
    let new_file = File::create(path)?;

    let value = to_serialized_scene(world, entity);
    serde_json::to_writer(new_file, &value.unwrap())?;

    Ok(())
}

pub fn load_scene(path: PathBuf, world: &mut World) -> Result<Entity, ()> {
    use std::io::prelude::*;
    let mut buf = String::new();
    let s = File::open(path).unwrap().read_to_string(&mut buf).unwrap();
    let deserialize = serde_json::from_str::<SerializedSceneData>(&buf).unwrap();
    let scene_component = deserialize.initialize(world).unwrap();

    Ok(world.spawn(scene_component).id())
}

/// Instantly despawns every entity belonging to the scene before despawning the scene entity.
///
/// Does not give any time or calls any methods on or before despawn.
/// If you want that, use a better (currently non-existent) method.
/// // fix that later maybe
///
/// Does not save before unloading! Make sure to call [`save_scene`] if anything in the scene must be serialized and stored.
/// If you don't have any non-global game state contained inside though, you're free to ignore that and unload as you please.
pub fn unload(scene_entity: Entity, world: &mut World) {
    for entity in world.get::<Scene>(scene_entity).unwrap().entities.clone() {
        world.despawn(entity.to_owned());
    }
    world.despawn(scene_entity);
}

/// Adds a new entity to the scene
///
/// Ensures that the component has a [`SceneData`] component. If it doesn't, then one gets added automatically.
pub fn add_entity_to_scene<'a>(
    world: &'a mut World,
    scene_entity: Entity,
    entity_to_add: Entity,
) -> Result<(), String> {
    if let None = World::entity(&world, entity_to_add).get::<traits::SceneData>() {
        world.entity_mut(entity_to_add).insert(traits::SceneData {
            object_name: String::from("New entity"),
            scene_id: 0, // TODO: Make use of scene_ids or get rid of them idk why i added them
        });
    }
    let mut scene_entity = World::entity_mut(world, scene_entity);
    let mut scene = scene_entity.get_mut::<Scene>().unwrap();

    scene.entities.push(entity_to_add.clone());

    Ok(())
}

// TODO: Fix docs
pub fn to_serialized_scene<'a>(
    world: &'a mut World,
    scene_entity: Entity,
) -> Result<serialize::SerializedSceneData, String> {
    let mut new_serialized_data: Vec<String> = Vec::new();
    let scene = world.entity(scene_entity);

    let entities_in_scene = scene.get::<Scene>().unwrap().entities.clone();

    let mut entities_serialized_count: u64 = 0;
    let mut components_serialized_count: u64 = 0;

    let mut serialized_data_from_entity: Vec<u8> = Vec::new();

    let mut typed_json = serde_json::Serializer::new(serialized_data_from_entity);

    {
        let mut erased_json = <dyn erased_serde::Serializer>::erase(&mut typed_json);
        for (entity, components) in world
            .query::<(Entity, &dyn traits::TestSuperTrait)>()
            .iter(world)
        {
            if !entities_in_scene.contains(&entity) {
                continue;
            }
            let obj_name = &world.get::<SceneData>(entity).unwrap().object_name;

            // erased_json.erased_serialize_str(obj_name);

            for component in components {
                component.erased_serialize(&mut erased_json);
                components_serialized_count += 1;
            }
            entities_serialized_count += 1;
        }
    }

    let serialized_data_from_entity = typed_json.into_inner();

    let checked_sdfe: String = match String::from_utf8(serialized_data_from_entity) {
        Ok(string) => string,
        Err(err) => return Err(err.to_string()),
    };

    new_serialized_data.push(checked_sdfe);

    println!(
        "Data: {:#?}, Entities serialized: {}, Components serialized: {}",
        new_serialized_data, entities_serialized_count, components_serialized_count
    );

    let mut scene = world.get_mut::<Scene>(scene_entity).unwrap();
    scene.serialized_entity_component_data = Some(new_serialized_data);
    Ok(serialize::SerializedSceneData {
        name: scene.name.clone(),
        entity_data: scene.serialized_entity_component_data.clone().unwrap(),
    })
}
