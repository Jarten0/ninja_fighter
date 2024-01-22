use std::fmt::Display;

use bevy_ecs::{
    bundle::Bundle,
    component::{Component, ComponentId},
    entity::Entity,
    system::{Command, Commands, EntityCommand, EntityCommands, Spawn},
    world::World,
};
use serde::Serialize;

#[derive(Debug, Bundle)]
pub struct SceneBundle {
    pub scene: Scene,
}

/// Entity managment for loading and unloading in batches rather than having everything loaded at once.
///
/// Each [`Entity`] must have the [`SceneData`] component if it wishes to be managed by a scene.
///
/// If you'd rather have your entities not managed by a scene, you can simply omit the [`SceneData`] component.
#[derive(Debug, Component)]
pub struct Scene {
    pub name: String,
    pub entities: Vec<Entity>,
}

struct LoadSceneCommand {
    pub scene_string: String,
}

struct SaveSceneCommand {
    pub scene: Entity,
}

// impl Command for LoadSceneCommand {
//     fn apply(self, world: &mut World) {
//         let bundle = SceneBundle {
//             scene: Scene::load(self.scene_string),
//         };
//         let spawn = world.spawn(bundle);
//     }
// }

/// Scene plaintext structure

trait ParseComponent {
    /// Add every field you wish to preserve
    fn field_names(&self) -> Vec<&str>;
}

/// SceneName
///     Object1
///         Component1
///
///     Object2
///         

impl Scene {
    pub fn new(name: String) -> Self {
        Self {
            name,
            entities: Vec::new(),
        }
    }

    pub fn load(scene_data: String, commands: &mut Commands) {
        let mut split = scene_data.lines().into_iter();

        let scene_name: &str = split.next().expect("Missing scene name/overall scene data");

        let scene_build = Scene::new(scene_name.to_owned());
        let scene_bundle = SceneBundle { scene: scene_build };

        commands.add(Spawn {
            bundle: scene_bundle,
        });

        loop {
            match split.next() {
                Some(line) => {}
                None => break,
            }
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
}

pub struct SceneTag {
    scene_name: String,
}

#[derive(bevy_ecs::component::Component)]
pub struct SceneData {
    pub root_scene: String,
    pub other_components: Vec<ComponentId>,
}
