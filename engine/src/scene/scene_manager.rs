use std::collections::HashMap;
use std::fmt::Debug;
use std::path::PathBuf;

use bevy_ecs::system::Resource;
use bevy_ecs::world::World;
use bevy_ecs::{entity::Entity, world::Mut};
use bevy_reflect::TypeRegistry;

use crate::scene::validate_name;

use super::{component::Scene, load_scene, save_scene, SceneError};

#[derive(Resource, Default)]
pub struct SceneManager {
    /// Contains every [`Entity`] with a [`Scene`]
    ///
    /// Key is equal to the scene name
    pub current_scenes: HashMap<String, Entity>,
    /// The current scene that's being prioritized for saving and loading
    pub target_scene: Option<Entity>,

    pub type_registry: TypeRegistry,
}

impl Debug for SceneManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SceneManager")
            .field("current_scenes", &self.current_scenes)
            .field("target_scene", &self.target_scene)
            .finish()
    }
}

impl SceneManager {
    pub fn new_scene(&self, world: &mut World, mut name: String) -> Result<(), SceneError> {
        let names: Vec<&String> = self.current_scenes.keys().collect();
        {
            let mut i = 0;
            loop {
                let mut contains = false;
                for name_already_entered in names.iter() {
                    if **name_already_entered == name {
                        contains = true;
                        break;
                    }
                }

                if contains == false {
                    break;
                }

                println!("{:?} contains {}", "Som", &name);

                let suffix = format!("({})", i);
                name.strip_suffix(&suffix);
                i += 1;
                name.push_str(&format!("({})", i))
            }
        };
        super::new_scene(world, name);

        todo!()
    }

    pub fn save_scene(&self, world: &mut World) -> Result<(), SceneError> {
        match self.target_scene {
            None => Err(SceneError::NoTargetScene),
            Some(scene) => save_scene(scene, world, &self.type_registry),
        }
    }

    pub fn load_scene(&mut self, world: &mut World, path: PathBuf) -> Result<Entity, SceneError> {
        let result = load_scene(path, world, &self.type_registry);

        if let Ok(entity) = result {
            let scene_name = world
                .get::<Scene>(entity)
                .ok_or(SceneError::LoadFailure(
                    "Failed to find the scene component on the newly instantiated scene",
                ))?
                .name
                .to_owned();
            self.current_scenes.insert(scene_name, entity);
            self.target_scene = Some(entity);
        }
        result
    }
}
