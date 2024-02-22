use std::fmt::Debug;
use std::path::PathBuf;

use bevy_ecs::system::Resource;
use bevy_ecs::world::World;
use bevy_ecs::{entity::Entity, world::Mut};
use bevy_reflect::TypeRegistry;

use super::{component::Scene, load_scene, save_scene, SceneError};

#[derive(Resource, Default)]
pub struct SceneManager {
    /// Contains every [`Entity`] with a [`Scene`]
    pub current_scenes: Vec<Entity>,
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
    pub fn save_scene(&self, world: &mut World) -> Result<(), SceneError> {
        match self.target_scene {
            None => return Err(SceneError::NoTargetScene),
            Some(scene) => save_scene(scene, world, &self.type_registry),
        }
    }

    pub fn load_scene(&mut self, world: &mut World, path: PathBuf) -> Result<Entity, SceneError> {
        let result = load_scene(path, world, &self.type_registry);

        if let Ok(ok) = result {
            self.current_scenes.push(ok);
            self.target_scene = Some(ok);
        }
        result
    }
}
