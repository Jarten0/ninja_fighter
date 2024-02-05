use super::serialize;
use super::traits;
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::system::Commands;
use bevy_ecs::world::World;
use serde::Serialize;
use std;
use std::env::current_dir;
use std::fs::File;
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
    pub name: String,
    pub entities: Vec<Entity>,
    /// When serializing a scene, the component data must be serialized before the `serialize()` component is run.
    /// Since `serialize()` can't take in a [`World`] during the functionality, it instead must be taken care of beforehand.
    /// This means the responsibility falls on the caller to take care of this before every serialization.
    /// To do so, a simple call to `Scene::`
    pub(crate) serialized_entity_component_data: Option<Vec<String>>,
    pub(crate) save_data_path: PathBuf,
}

impl Scene {
    /// Creates a new blank [`Scene`] using the provided name.
    ///
    /// Does not contain any [`Entity`]'s and does not load any. To do that, wait until its ready.
    ///
    ///
    /// // TODO: When done, update these docs
    pub fn new(name: String) -> Self {
        Self {
            name,
            entities: Vec::new(),
            serialized_entity_component_data: None,
            save_data_path: PathBuf::new(),
        }
    }

    /// Instantly despawns every entity belonging to the scene before despawning the scene entity.
    ///
    /// Does not give any time or calls any methods on or before despawn.
    /// If you want that, use a better (currently non-existent) method.
    /// // fix that later maybe
    ///
    /// Does not save before unloading! Make sure to call `save()` if anything in the scene must be serialized and stored.
    /// If you don't have any non-global game state contained inside though, you're free to ignore that and unload as you please.
    pub fn unload(&self, commands: &mut Commands) {
        for entity in &self.entities {
            commands.entity(entity.to_owned()).despawn();
        }
    }

    /// Serializes all of the
    pub fn save(&self, world: &mut World) -> Result<(), std::io::Error> {
        let mut path = PathBuf::new();
        path.push(current_dir()?);
        let new_file = File::create(path)?;

        // serde_json::to_writer(new_file, self)?;

        Ok(())
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

pub fn add_entity_to_scene<'a>(
    world: &'a mut World,
    scene_entity: Entity,
    entity_to_add: Entity,
) -> Result<(), String> {
    let scene_data = World::entity(&world, entity_to_add).get::<traits::SceneData>();
    if let None = scene_data {
        world.entity_mut(entity_to_add).insert(traits::SceneData {
            root_scene: String::from("scene.name.clone()"), //man i aint dealin with that rn
            scene_id: 0, // TODO: Make use of scene_ids or get rid of them idk why i added them
            serializeable_components: Vec::new(),
        });
    }
    let mut scene_entity = World::entity_mut(world, scene_entity);
    let mut scene = scene_entity.get_mut::<Scene>().unwrap();

    scene.entities.push(entity_to_add.clone());

    Ok(())
}

/// Takes all of the entities from [`Scene`], takes the [`SceneData`] of each one, and iterates over every [`Component`] that is designated to be serialized.
/// It then serializes each component and stores that data as `JSON` inside of the `Scene`'s `serialized_entity_component_data` as a [`String`].
///
/// Later, when serializing the `Scene` itself, it will pull from there and store that data inside of the new serialized [`SerializedScene`] created.
///
/// Be careful when calling, as every call will overwrite the previous save data.
pub fn to_serialized_scene<'a>(
    world: &'a mut World,
    scene_entity: Entity,
) -> Result<serialize::SerializedSceneData, String> {
    let mut new_serialized_data: Vec<String> = Vec::new();

    let scene = world.entity_mut(scene_entity);

    let vec = scene.get::<Scene>().unwrap().entities.clone();

    for entity in &vec {
        let entity_scene_data = World::entity(&world, entity.to_owned())
            .get::<traits::SceneData>()
            .ok_or(format!(
                "Entity {:?} does not have a SceneData component!",
                entity
            ))?;

        let mut serialized_data_from_entity: Vec<u8> = Vec::new();

        let typed_json = &mut serde_json::Serializer::new(serialized_data_from_entity.clone());
        let mut erased_json = <dyn erased_serde::Serializer>::erase(typed_json);

        // TODO: FIX
        for component_id in &entity_scene_data.serializeable_components {
            let component_ptr = world
                .get_by_id(entity.to_owned(), component_id.to_owned())
                .unwrap();
            let component = todo!();
            // unsafe { Box::new(component_ptr.deref::<dyn erased_serde::Serialize>()) };
            let serialized_entity =
                erased_serde::Serialize::erased_serialize(&component, &mut erased_json);

            if let Err(value) = serialized_entity {
                eprintln!("Component failed to serialize! [{}]", value);
                return Err(value.to_string());
            };
        }

        let checked_sdfe: String = match String::from_utf8(serialized_data_from_entity) {
            Ok(string) => string,
            Err(err) => return Err(err.to_string()),
        };

        new_serialized_data.push(checked_sdfe);
    }

    let mut scene = world.get_mut::<Scene>(scene_entity).unwrap();
    scene.serialized_entity_component_data = Some(new_serialized_data);
    Ok(serialize::SerializedSceneData {
        name: scene.name.clone(),
        entity_data: scene.serialized_entity_component_data.clone().unwrap(),
    })
}

impl PartialEq for Scene {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
