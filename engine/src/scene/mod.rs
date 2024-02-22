mod component;
mod scene_object;
mod serialized_scene;

#[cfg(test)]
mod test;

mod scene_manager;
mod traits;

pub use component::{
    add_entity_to_scene, load_scene, save_scene, to_serialized_scene, unload_scene,
};

pub fn register_scene_types(world: &mut bevy_ecs::world::World) {
    world.init_resource::<scene_manager::SceneManager>();
    world.init_component::<traits::SceneData>();
}
