//! The [`Scene`] component, which allows managment of loading and unloading entities and components dynamically.
//! It can also serialize and deserialize component data and instantiate entities using it to provide full building functionality.

use crate::assets::Asset;
use crate::assets::AssetID;
use crate::assets::AssetStorage;
use crate::assets::SceneAssetID;
use crate::assets::SerializableAsset;
use crate::assets::SerializedAsset;
use crate::scene::object_data::SceneData;
use crate::scene::serialized_scene::ComponentData;
use crate::scene::serialized_scene::SerializedSceneData;
use crate::GgezInterface;

use super::error;
use super::object_data;
use super::object_id::ComponentInstanceID;
use super::serialized_scene;
use super::serialized_scene::DataHashmap;
use super::serialized_scene::EntityHashmap;
use super::CounterType;
use super::IDCounter;
use super::ObjectID;
use super::SceneError;
use super::SceneManager;
use super::TestSuperTrait;

use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::query::QueryEntityError;
use bevy_ecs::system::Query;
use bevy_ecs::world::World;

use bevy_reflect::serde::ReflectSerializer;
use bevy_reflect::serde::Serializable;
use bevy_reflect::GetTupleStructField;
use bevy_reflect::Reflect;
use bevy_reflect::ReflectRef;
use bevy_reflect::ReflectSerialize;
use bevy_reflect::TypeRegistry;
use inquire::Text;
use log::error;
use log::trace;
use object_data::CustomSerialization;
use serde::Serialize;
use serde_json::Value;

use std::any::Any;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::File;
use std::fs::OpenOptions;
use std::hash::Hash;
use std::hash::Hasher;
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
#[derive(Debug, Component)]
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
    /// The path where the scene's save data is stored when calling [`save_scene`]  
    pub(crate) save_data_path: Option<PathBuf>,

    pub scene_id: ObjectID,

    pub(crate) assets: HashMap<SceneAssetID, Asset<Box<dyn Reflect>>>,
}

impl Clone for Scene {
    fn clone(&self) -> Self {
        let assets = {
            let mut assets = HashMap::new();
            for (id, asset) in self.assets.iter() {
                let new_asset = Asset::new(
                    asset.asset_name.clone(),
                    asset.asset_data.clone_value(),
                    asset.storage.clone(),
                );
                assets.insert(SceneAssetID::get(new_asset.asset_name.as_str()), new_asset);
            }
            assets
        };
        Self {
            name: self.name.clone(),
            entities: self.entities.clone(),
            save_data_path: self.save_data_path.clone(),
            scene_id: self.scene_id.clone(),
            assets,
        }
    }
}

impl Scene {
    /// Creates a new blank [`Scene`] using the provided name.
    pub fn new(name: String) -> Self {
        Self {
            name,
            entities: Vec::new(),
            save_data_path: None,
            scene_id: ObjectID::new(CounterType::Scenes),
            assets: HashMap::new(),
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
                    .get::<object_data::SceneData>()
                    .unwrap()
                    .scene_id
            } else {
                return false;
            };

            let other_entity = if let Some(entity) = self.entities.get(i) {
                world
                    .entity(entity.to_owned())
                    .get::<object_data::SceneData>()
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

    pub fn get_entity(&self, world: &World, name: String) -> Option<Entity> {
        for entity in &self.entities {
            if world.get::<SceneData>(*entity).unwrap().entity_name == name {
                return Some(*entity);
            }
        }
        None
    }

    pub fn get_entities(&self) -> &Vec<Entity> {
        &self.entities
    }

    pub fn save_data_path(&self) -> Option<&PathBuf> {
        self.save_data_path.as_ref()
    }

    pub fn get_asset(&self, id: &SceneAssetID) -> Option<&Asset<Box<dyn Reflect>>> {
        self.assets.get(id)
    }

    pub fn iter_assets(
        &self,
    ) -> std::collections::hash_map::Iter<'_, SceneAssetID, Asset<Box<dyn Reflect>>> {
        self.assets.iter()
    }

    /// Stores a reflectable asset inside of the scene.
    ///
    /// If you want an asset to be stored somewhere else, use the corresponding function.
    pub fn create_asset(&mut self, asset_name: String, asset_data: Box<dyn Reflect>) {
        let storage = AssetStorage::Scene(self.get_scene_id_from_name(asset_name.as_str()));
        let asset = Asset::new(asset_name, asset_data, storage);
        self.assets
            .insert(SceneAssetID::get(asset.asset_name.as_str()), asset);
    }

    /// Gets the serialization info for an asset for saving and loading data.
    pub(crate) fn get_asset_with_serialize_info<'asset, 'registry>(
        &'asset mut self, // the asset lives for as long as the scene does
        type_registry: &'registry TypeRegistry,
        asset_id: SceneAssetID,
    ) -> Option<SerializableAsset<'asset, 'registry>> {
        let asset = self.assets.get(&asset_id)?;
        Some(SerializableAsset::from_reflect_asset(asset, type_registry))
    }

    pub(crate) fn initialize_asset(&mut self, asset_data: serde_json::Value) {
        // let asset = Asset::new(asset_data);
        todo!(); // TODO
                 // self.assets.insert(asset.id, asset);
    }

    pub fn get_scene_id_from_name(&self, asset_name: &str) -> SceneAssetID {
        let mut hasher = std::hash::DefaultHasher::new();
        asset_name.hash(&mut hasher);
        SceneAssetID(hasher.finish().try_into().unwrap())
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
pub fn save_scene(
    entity: Entity,
    world: &mut World,
    registry: &TypeRegistry,
) -> Result<(), error::SceneError> {
    trace!("Saving scene to file");

    let scene = world
        .get::<Scene>(entity)
        .ok_or(SceneError::NoSceneComponent)?;

    let scene_name = scene.name.clone();

    let f = || -> Result<PathBuf, SceneError> {
        let value = world
            .resource::<GgezInterface>()
            .get_engine_config()
            .scenes_folder
            .or_else(|| todo!())
            .ok_or(SceneError::NoEntitiesAvailable)?; // todo pick better error

        let mut from = PathBuf::from(value);

        from.push(scene_name + ".json");

        Ok(from)
    };

    let path = match scene.save_data_path.clone().ok_or_else(f) {
        Ok(ok) => ok,
        Err(err) => err?,
    };

    trace!("Checking file extension");

    if !(path.extension().unwrap_or(OsStr::new("none")) == "json") {
        return Err(SceneError::SerializeFailure(format!(
            "Invalid file type: Must be JSON. [{}]",
            path.display()
        )));
    }

    trace!("Found path");

    let save_file = match OpenOptions::new()
        .read(false)
        .write(true)
        .create(true)
        .open(path.clone())
    {
        Ok(ok) => {
            trace!("A file was found! Will overwrite the found file after serializing scene.");
            ok
        }
        Err(err) => {
            trace!("No file found, creating new file. [{}]", err);
            match File::create(path.clone()) {
                Ok(ok) => ok,
                Err(err) => return Err(error::SceneError::IOError(err)), //Could the target scene's path be incorrect?)
            }
        }
    };

    let value = create_serializable_scene_data(world, registry, entity)
        .map_err(|err| SceneError::SerializeFailure(err.to_string()))?;

    trace!("Writing saved data to disk");

    save_file.set_len(0);

    serde_json::to_writer_pretty(save_file, &value)
        .map_err(|err| error::SceneError::SerializeFailure(err.to_string()))?;

    world
        .get_mut::<Scene>(entity)
        .ok_or(SceneError::NoSceneComponent)?
        .save_data_path = Some(path);

    trace!("Saved scene successfully");

    Ok(())
}

/// Creates a new scene component and spawns an entity with it
///
/// Does no validation to check if it shares a name, that's on the caller of the function
pub fn new_scene(world: &mut World, name: String) -> Entity {
    let scene = Scene::new(name);
    let entity = world.spawn(scene).id();
    entity
}

pub fn load_scene(
    path: PathBuf,
    world: &mut World,
    registry: &TypeRegistry,
) -> Result<Entity, error::SceneError> {
    use std::io::prelude::*;

    trace!("Opening file");

    let mut buf = String::new();
    let _s = File::open(path.clone())
        .map_err(|err| -> error::SceneError { error::SceneError::IOError(err) })?
        .read_to_string(&mut buf)
        .map_err(|err| SceneError::IOError(err))?;

    trace!("File found");

    let deserialize = serde_json::from_str::<SerializedSceneData>(&buf).map_err(|err| {
        error::SceneError::LoadFailure(
            "SerializedSceneData failed to deserialize: ".to_string() + err.to_string().as_str(),
        )
    })?;

    let scene_entity = deserialize.initialize(world, registry)?;

    trace!(
        "Loaded scene {} successfully",
        path.to_str()
            .expect("expected the path to be valid unicode")
    );

    world.get_mut::<Scene>(scene_entity).unwrap().save_data_path = Some(path);

    Ok(scene_entity)
}

/// Instantly despawns every entity belonging to the scene before despawning the scene entity.
///
/// Does not give any time or calls any methods on or before despawn.
/// If you want that, use a better (currently non-existent) method.
/// // fix that later maybe
///
/// Does not save before unloading! Make sure to call [`save_scene`] if anything in the scene must be serialized and stored.
/// If you don't have any non-global game state contained inside though, you're free to ignore that and unload as you please.
pub fn unload_scene(scene_entity: Entity, world: &mut World) -> Result<(), SceneError> {
    for entity in world
        .get::<Scene>(scene_entity)
        .ok_or(SceneError::NoSceneComponent)?
        .entities
        .clone()
    {
        world.despawn(entity.to_owned());
    }
    world.despawn(scene_entity);

    Ok(())
}

/// Adds a new entity to the scene
///
/// This function takes several steps to validate the entity before appending to the list.
/// * Ensures that the component has a [`SceneData`] component. If it doesn't, then one gets added automatically.
/// * Checks that no other entity in the scene has the same [`SceneData`] name. If there is, it will be adjusted to ensure absolute uniqueness of names.
pub fn add_entity_to_scene<'a>(
    world: &'a mut World,
    scene_entity: Entity,
    entity_to_add: Entity,
    name: Option<String>,
) -> Result<(), SceneError> {
    // Very specific way this following code is blocked, since we need a list of entity names that DOESN'T include the entity currently being added
    let mut entity_names: Vec<String> = Vec::new();
    for (component, entity) in world.query::<(&SceneData, Entity)>().iter(world) {
        if !world
            .get::<Scene>(scene_entity)
            .ok_or(SceneError::NoSceneComponent)?
            .entities
            .contains(&entity)
        {
            continue;
        }
        entity_names.push(component.entity_name.to_owned());
    }

    if let None = world.get::<SceneData>(entity_to_add) {
        let object_name = match name {
            Some(some) => some,
            None => String::from("New entity"),
        };
        let scene_id = world
            .get::<Scene>(scene_entity)
            .ok_or(SceneError::NoSceneComponent)?
            .scene_id;

        let mut component_paths = HashMap::new();
        let mut component_ids = HashMap::new();

        let mut query = world.query::<&dyn TestSuperTrait>();

        match query.get(world, entity_to_add) {
            Ok(components) => {
                for component in components.iter() {
                    let id = ComponentInstanceID::get_new();
                    let path = component.as_reflect().reflect_type_path().to_string();
                    component_paths.insert(id, path.clone());
                    component_ids.insert(path, id);
                }
                trace!("Added inspectable components from entity to scene");
            }
            Err(err) => {
                let QueryEntityError::QueryDoesNotMatch(err) = err else {
                    return Err(SceneError::QueryFailure(err.to_string()));
                };
                trace!("No inspectable components found, adding effectively empty entity to scene");
            }
        };

        world
            .entity_mut(entity_to_add)
            .insert(object_data::SceneData {
                entity_name: object_name,
                scene_id: Some(scene_id),

                hide_in_inspector: true,
            });
    }

    validate_name(
        &mut entity_names.iter(),
        &mut world
            .get_mut::<SceneData>(entity_to_add)
            .ok_or(SceneError::NoSceneDataComponent)?
            .entity_name,
    );

    let mut scene_entity = World::entity_mut(world, scene_entity);
    let mut scene = scene_entity
        .get_mut::<Scene>()
        .ok_or(SceneError::NoSceneComponent)?;

    scene.entities.push(entity_to_add.clone());

    Ok(())
}

/// Runs through the list of names, and checks to see if the name is a duplicate of any inside the list
///
/// If it is, the function will automatically append a new ID and try again.
///
/// Operation is currently worst case of `O(n^2)`
pub fn validate_name(names: &mut dyn Iterator<Item = &String>, name_to_check: &mut String) {
    let mut i = 0;
    loop {
        let mut contains = false;
        for name in &mut *names {
            if name == name_to_check {
                contains = true;
                break;
            }
        }

        if contains == false {
            break;
        }

        println!("{:?} contains {}", "Som", &name_to_check);

        let suffix = format!(" ({})", i);
        *name_to_check = name_to_check
            .strip_suffix(&suffix)
            .unwrap_or(&name_to_check)
            .to_owned();
        i += 1;
        name_to_check.push_str(&format!(" ({})", i))
    }
}

// TODO: Create better documentation, this is one of the most important functions to do so for
/// Creates a [`SerializableScene`] using the scene's component data
pub fn create_serializable_scene_data<'a>(
    world: &'a mut World,
    type_registry: &TypeRegistry,
    scene_entity: Entity,
) -> Result<serialized_scene::SerializedSceneData, SceneError> {
    let scene = world
        .entity(scene_entity)
        .get::<Scene>()
        .ok_or(SceneError::NoSceneComponent)?;

    let scene_name = scene.name.clone();

    let scene_entity_list: Vec<Entity> = scene.entities.clone();

    trace!("Serializing scene {}", scene_name);

    // scene reference is dropped here

    let mut entity_data: HashMap<String, HashMap<String, Value>> = HashMap::new();

    for (entity, serializable_components_data) in world
        .query::<(Entity, &dyn object_data::TestSuperTrait)>()
        .iter(world)
    {
        if !scene_entity_list.contains(&entity) {
            continue;
        }

        let entity_name = world.get::<SceneData>(entity).unwrap().entity_name.clone();

        trace!("  - Serializing {}'s components", entity_name);

        let mut entity_hashmap: HashMap<String, Value> = HashMap::new();

        for component in serializable_components_data.iter() {
            let reflected_component = component.as_reflect();

            let component_type_path = reflected_component.reflect_type_path();
            trace!("    - Serializing {}", component_type_path);

            let Some(registration) = type_registry.get(component.as_reflect().type_id()) else {
                log::error!(
                    "Could not find type registration for {}",
                    component.as_reflect().reflect_type_path()
                );
                continue;
            };

            let Some(reflect_serialize) = registration.data::<ReflectSerialize>() else {
                log::error!(
                    "Could not find ReflectSerialize type data for {}",
                    component.as_reflect().reflect_type_path()
                );
                continue;
            };

            let serializable_component: Serializable =
                reflect_serialize.get_serializable(component.as_reflect());

            let value = match serializable_component {
                Serializable::Owned(owned) => serde_json::to_value(owned),
                Serializable::Borrowed(borrowed) => serde_json::to_value(borrowed),
            }
            .unwrap();

            trace!("      - Serialized");

            entity_hashmap.insert(reflected_component.reflect_type_path().to_owned(), value);

            trace!(
                "   - Inserted serialized component data to {}'s serialize data",
                entity_name
            );
        }
        let k = world.get::<SceneData>(entity).unwrap().entity_name.clone();

        entity_data.insert(k, entity_hashmap);
    }

    // Serialize empty entities aswell
    for entity in scene_entity_list {
        // Try/Catch to prevent failure from entities with missing scene components
        let result = || -> Result<(), SceneError> {
            let object_name = &world
                .get::<SceneData>(entity)
                .ok_or(SceneError::NoSceneDataComponent)?
                .entity_name;

            if !entity_data.contains_key(object_name) {
                entity_data.insert(object_name.to_owned(), HashMap::new());
            };
            Ok(())
        }()
        .map_err(|err| err.to_string());

        if let Err(err) = result {
            error!("Skipped serializing entity {}: [{}]", entity.index(), err)
        }
    }

    trace!("Serialized entities successfully.");

    trace!("Serializing assets...");

    let mut asset_data = HashMap::new();

    let scene = world
        .entity(scene_entity)
        .get::<Scene>()
        .ok_or(SceneError::NoSceneComponent)?;

    for (_, asset) in &scene.assets {
        trace!("Serializing {}", &asset.asset_name);
        let name = asset.asset_name.clone();
        let serializable = SerializableAsset::from_reflect_asset(asset, &type_registry); // type registry here is borrowed for as long as `seriailzable` exists
        let serialized = serde_json::to_value(serializable).unwrap();
        asset_data.insert(name, serialized);
    }

    trace!("Serialized assets successfully.");

    trace!("Serialized scene {} successfully.", scene_name);

    Ok(serialized_scene::SerializedSceneData {
        name: scene_name,
        entity_data,
        asset_data,
    })
}

fn check_string(
    serialized_data_from_entity: Vec<u8>,
) -> Result<String, Result<SerializedSceneData, String>> {
    Ok(match String::from_utf8(serialized_data_from_entity) {
        Ok(string) => string,
        Err(err) => return Err(Err(err.to_string())),
    })
}
