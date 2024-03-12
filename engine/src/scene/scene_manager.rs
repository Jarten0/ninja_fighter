use std::collections::HashMap;
use std::fmt::Debug;
use std::path::PathBuf;

use bevy_ecs::entity::Entity;
use bevy_ecs::system::Resource;
use bevy_ecs::world::World;
use bevy_reflect::TypeRegistry;

use super::{component::Scene, load_scene, save_scene};
use super::{error, unload_scene, ObjectID};

#[derive(Resource, Default)]
pub struct SceneManager {
    /// Contains every [`Entity`] with a [`Scene`]
    ///
    /// Key is equal to the scene name
    pub current_scenes: HashMap<String, Entity>,
    pub scenes_by_ids: HashMap<ObjectID, Entity>,
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
    pub fn new_scene(
        &mut self,
        world: &mut World,
        mut name: String,
    ) -> Result<(), error::SceneError> {
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
                name = name.strip_suffix(&suffix).unwrap_or("default").to_owned();
                i += 1;
                name.push_str(&format!("({})", i))
            }
        };
        let new_scene = super::new_scene(world, name.clone());
        self.current_scenes.insert(name, new_scene);
        self.scenes_by_ids
            .insert(world.get::<Scene>(new_scene).unwrap().scene_id, new_scene);
        self.target_scene = Some(new_scene);
        Ok(())
    }

    pub fn save_scene(&self, world: &mut World) -> Result<(), error::SceneError> {
        match self.target_scene {
            None => Err(error::SceneError::NoTargetScene),
            Some(scene) => save_scene(scene, world, &self.type_registry),
        }
    }

    pub fn load_scene(
        &mut self,
        world: &mut World,
        path: PathBuf,
    ) -> Result<Entity, error::SceneError> {
        let result = load_scene(path, world, &self.type_registry);

        if let Ok(entity) = result {
            let scene_name = world
                .get::<Scene>(entity)
                .ok_or(error::SceneError::LoadFailure(
                    "Failed to find the scene component on the newly instantiated scene".to_owned(),
                ))?
                .name
                .to_owned();
            self.current_scenes.insert(scene_name, entity);
            self.scenes_by_ids
                .insert(world.get::<Scene>(entity).unwrap().scene_id, entity);

            self.target_scene = Some(entity);
        }
        result
    }

    pub fn unload_scene(&mut self, world: &mut World) -> Result<(), error::SceneError> {
        if let Some(target_scene) = self.target_scene.take() {
            Ok(unload_scene(target_scene, world))
        } else {
            Err(error::SceneError::NoTargetScene)
        }
    }

    pub fn get_scene_by_id(&self, id: ObjectID) -> Option<Entity> {
        self.scenes_by_ids.get(&id).copied()
    }
    pub fn get_scene_by_name(&self, name: String) -> Option<Entity> {
        self.current_scenes.get(&name).copied()
    }
}
