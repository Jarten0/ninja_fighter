use std::fmt::Display;

use bevy_ecs::{
    bundle::Bundle,
    component::{Component, ComponentId},
    entity::Entity,
    system::{Command, Commands, EntityCommand, EntityCommands, Spawn},
    world::World,
};
use serde::{de::Visitor, ser::SerializeStruct, Deserialize, Serialize};

use std::{collections::HashMap, hash::Hash, sync::atomic::AtomicUsize};

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
}

impl PartialEq for Scene {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[derive(Debug)]
pub struct SerializedScene {
    pub name: String,
    pub entity_data: String,
}

impl SerializedScene {
    pub fn initialize(self, world: &mut World) -> Scene {
        let scene = Scene::new(self.name.to_owned());
        // TODO: Deserialize entity data here and add it to the scene
        scene
    }
}

impl Scene {
    pub fn new(name: String) -> Self {
        Self {
            name,
            entities: Vec::new(),
        }
    }

    pub fn unload(&self, commands: &mut Commands) {
        for entity in &self.entities {
            commands.entity(entity.to_owned()).despawn();
        }
    }

    pub fn save(&self, world: &mut World) {
        let entity = self.entities.get(0).unwrap().to_owned();
        world.entity(entity);
    }

    /// The comparison operator (`==`) but with entity comparison.
    /// This checks every entity to see if their [`SceneData`] matches, and if not, returns `false`.
    ///
    /// If you don't want to compare [`Scene`]s with world access, you can use the default `==` operator implementation.
    /// However, it will only compare the scene's IDs, so entity checking is not viable.
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

impl Serialize for Scene {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut serialize_struct = serializer.serialize_struct("SerializedScene", 2)?;
        serialize_struct.serialize_field("name", &self.name);
        // let value = &self.entities;
        serialize_struct.serialize_field("entity_data", &String::new()); // TODO: Convert entites to string data that can be deserialized
        serialize_struct.end()
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

struct SceneVisitor;
impl Visitor<'_> for SceneVisitor {
    type Value = SerializedScene;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a serialized scene with an entities field")
    }
}

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

#[test]
fn scene_test() {
    let mut world = World::new();

    let init_scene: Scene = dbg!(Scene {
        name: String::from("TestScene"),
        entities: Vec::new(),
    });

    let to_string: String =
        dbg!(serde_json::to_string(&init_scene).expect("Jesoon parse error, failed on serialize"));

    let serialized_scene: SerializedScene =
        dbg!(serde_json::from_str::<SerializedScene>(&to_string)
            .expect("Jesoon parse error, failed on deserialize"));

    let result_scene: Scene = dbg!(SerializedScene::initialize(serialized_scene, &mut world));

    assert!(Scene::entity_eq(&init_scene, &result_scene, &mut world))
}

pub struct SceneTag {
    scene_name: String,
}

#[derive(bevy_ecs::component::Component)]
pub struct SceneData {
    pub root_scene: String,
    pub other_components: Vec<ComponentId>,
    pub scene_id: usize,
}
