use std::{
    env::current_dir,
    fmt::Display,
    fs::File,
    io::{self, BufWriter, Write},
    ops::{Deref, DerefMut},
    path::PathBuf,
};

use bevy_ecs::{
    bundle::Bundle,
    component::{Component, ComponentId},
    entity::Entity,
    system::{Command, Commands, EntityCommand, EntityCommands, Resource, Spawn},
    world::World,
};
use erased_serde::Serialize as ErasedSerialize;
use serde::{de::Visitor, ser::SerializeStruct, Deserialize, Serialize};

use std::{collections::HashMap, hash::Hash, sync::atomic::AtomicUsize};

use crate::{scene, space::transform};

#[derive(Debug, Bundle)]
pub struct SceneBundle {
    pub scene: Scene,
}

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
    serialized_entity_component_data: Option<Vec<String>>,
    save_data_path: PathBuf,
}

impl Scene {
    /// Creates a new blank [`Scene`] using the provided name.
    ///
    /// Does not contain any [`Entity`]'s and does not load any. To do that, wait until its ready.
    ///
    ///
    // TODO: When done, update these docs
    pub fn new(name: String) -> Self {
        Self {
            name,
            entities: Vec::new(),
            serialized_entity_component_data: None,
            save_data_path: PathBuf::new(),
        }
    }

    pub fn add_entity<'a>(&mut self, entity: Entity, world: &'a World) -> Result<(), String> {
        let get = world.entity(entity).get::<SceneData>();
        if let None = get {
            return Err(String::from("Entity does not have SceneData"));
        } else {
        }

        self.entities.push(entity.clone());

        Ok(())
    }

    /// Instantly despawns every entity belonging to the scene before despawning the scene entity.
    ///
    /// Does not give any time or calls any methods on or before despawn.
    /// If you want that, use a better (currently non-existent) method.
    // fix that later maybe
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

        serde_json::to_writer(new_file, self)?;

        Ok(())
    }

    /// Takes all of the entities from [`Scene`], takes the [`SceneData`] of each one, and iterates over every [`Component`] that is designated to be serialized.
    /// It then serializes each component and stores that data as `JSON` inside of the `Scene`'s `serialized_entity_component_data` as a [`String`].
    ///
    /// Later, when serializing the `Scene` itself, it will pull from there and store that data inside of the new serialized [`SerializedScene`] created.
    ///
    /// Be careful when calling, as every call will overwrite the previous save data.
    pub fn serialize_entity_components<'a>(&'a mut self, world: &'a World) -> Result<(), String> {
        let mut new_serialized_data: Vec<String> = Vec::new();

        for entity in &self.entities {
            let scene_data = world.get::<SceneData>(entity.to_owned()).ok_or(format!(
                "Entity {:?} does not have a SceneData component!",
                entity
            ))?;

            let mut inner_file: Vec<u8> = Vec::new();

            let typed_json = &mut serde_json::Serializer::new(inner_file.clone());
            let mut erased_json = <dyn erased_serde::Serializer>::erase(typed_json);

            for component_id in &scene_data.serializeable_components {
                let serialized_entity =
                    Self::serialize_entity(component_id, &mut inner_file, &mut erased_json);
                if let Err(value) = serialized_entity {
                    eprintln!("Component failed to serialize! [{}]", value);
                    return Err(value);
                };
            }

            let string: String = match String::from_utf8(inner_file) {
                Ok(string) => string,
                Err(err) => return Err(err.to_string()),
            };

            new_serialized_data.push(string);
        }

        self.serialized_entity_component_data = Some(new_serialized_data);

        Ok(())
    }

    /// Finds a file, creates a buffer to that file, gives it to the serializer, has the serializer write into the buffer, then flushes the buffer into the file.
    /// ez
    fn serialize_entity(
        component_id: &Box<dyn SerializableComponent>,
        writer: &mut impl Write,
        erased_json: &mut impl erased_serde::Serializer,
    ) -> Result<(), String> {
        let erased_serialize = ErasedSerialize::erased_serialize(&component_id, erased_json);
        if let Err(err) = erased_serialize {
            return Err(err.to_string());
        };

        Ok(())
    }

    /// The comparison operator (`==`) but with entity comparison.
    /// This checks every entity to see if their [`SceneData`] matches, and if not, returns `false`.
    ///
    /// If you don't want to compare [`Scene`]s with world access, you can use the default `==` operator implementation.
    /// However, it will only compare the scene's IDs, so entity checking is not viable.'/:`12`
    pub fn entity_eq(&self, other: &Scene, world: &mut World) -> bool {
        if self.name != other.name {
            return false;
        }

        for i in 0..self.entities.len() {
            let self_entity = if let Some(entity) = other.entities.get(i) {
                world
                    .entity(entity.to_owned())
                    .get::<SceneData>()
                    .unwrap()
                    .scene_id
            } else {
                return false;
            };

            let other_entity = if let Some(entity) = self.entities.get(i) {
                world
                    .entity(entity.to_owned())
                    .get::<SceneData>()
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
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Serialize for Scene {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let entity_json = serde_json::to_string(&self.entities).unwrap();
        // TODO: Convert entites to string data that can be deserialized

        let mut serialize_struct = serializer.serialize_struct("SerializedScene", 2)?;

        serialize_struct.serialize_field("name", &self.name);
        serialize_struct.serialize_field("entity_data", &entity_json);
        serialize_struct.end()
    }
}

/// A string based data type that stores useful data to convert [`Scene`]'s and bevy_ecs [`Entity`]'s to strings and back.
#[derive(Debug)]
pub struct SerializedScene {
    pub name: String,
    pub entity_data: String,
}

impl SerializedScene {
    pub fn initialize(self, world: &mut World) -> serde_json::Result<Scene> {
        let scene = Scene::new(self.name.to_owned());
        // TODO: Deserialize entity data here and add it to the scene

        let v: serde_json::Value = serde_json::from_str(&self.entity_data)?;
        let entity = world.spawn_empty();
        // for component in v.

        dbg!(v);

        Ok(scene)
    }
}

impl<'de> Deserialize<'de> for SerializedScene {
    /// Takes in a [`SerializedScene`] struct made from [`Scene`]`::serialize()` and turns it back into a scene
    ///
    /// Required to call [`SerializedScene`]::initialize() to turn it back into a useable [`Scene`]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_struct("SerializedScene", &["name", "entity_data"], SceneVisitor)
    }
}

/// Simple [`Visitor`] for deserializing [`SerializedScene`]'s from `Jesoon` (or whatever serializer is used) into a proper Rust data type
struct SceneVisitor;

impl<'de> Visitor<'de> for SceneVisitor {
    type Value = SerializedScene;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a serialized scene with an entities field")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut serialized_scene = SerializedScene {
            name: String::new(),
            entity_data: String::new(),
        };

        // This at one point was like 20 lines long
        while let Some((key)) = map.next_key::<String>()? {
            match key.as_str() {
                "name" => serialized_scene.name = map.next_value()?,
                "entity_data" => serialized_scene.entity_data = map.next_value()?,
                _ => (),
            };
        }
        Ok(serialized_scene)
    }
}

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
}

erased_serde::serialize_trait_object!(SerializableComponent);

pub struct SerializedComponent {}

// TODO: figure this struct out or remove it
#[derive(Debug, Eq, Clone, Copy)]
pub struct SceneObjectID {
    pub(crate) id: usize,
}

impl PartialEq for SceneObjectID {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for SceneObjectID {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl SceneObjectID {
    pub fn get_id() -> usize {
        pub(crate) static COUNTER: AtomicUsize = AtomicUsize::new(1);
        COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    }
}

/// Holds data for the assigned [`Scene`] to operate upon.
/// An entity cannot be serialized by the [`Scene`] if it does not have this component.
///
// TODO: finish [`SceneData`] docs
#[derive(bevy_ecs::component::Component)]
pub struct SceneData {
    pub root_scene: String,
    /// The ID of the current scene that the component holder belongs to.
    /// Not to be confused with the [`SceneObjectID`], which is a seperate thing uhh
    // TODO: figure that out
    pub scene_id: usize,
    pub serializeable_components: Vec<Box<dyn SerializableComponent>>,
}

#[test]
fn scene_test() {
    let mut world = World::new();
    let test_entity = world.spawn(transform::Transform::default()).id().clone();
    world.entity_mut(test_entity).insert(SceneData {
        root_scene: String::new(),
        scene_id: 0,
        serializeable_components: Vec::new(),
    });

    let mut scene: Scene = dbg!(Scene::new("TestScene".to_string()));

    if let Err(err) = Scene::add_entity(&mut scene, test_entity, &world) {
        panic!("Adding entity failed! [{}]", err)
    }

    if let Err(err) = Scene::serialize_entity_components(&mut scene, &world) {
        panic!("Serializing entity failed! [{}]", err)
    }

    let bundle = dbg!(SceneBundle { scene });

    let mut scene_entity = World::spawn(&mut world, bundle);

    let to_string: String = dbg!(serde_json::to_string(&scene_entity.get::<Scene>())
        .expect("jesoon should have serialized properly"));

    let serialized_scene: SerializedScene =
        dbg!(serde_json::from_str::<SerializedScene>(&to_string.clone())
            .expect("jesoon should have deserialized properly"));

    let result_scene: Scene =
        dbg!(SerializedScene::initialize(serialized_scene, &mut world).unwrap());

    let mut path_buf = current_dir().unwrap();
    path_buf.push("test_output.txt");
    File::create(path_buf).unwrap().write(to_string.as_bytes());
    // assert!(Scene::entity_eq(&init_scene, &result_scene, &mut world))
}
