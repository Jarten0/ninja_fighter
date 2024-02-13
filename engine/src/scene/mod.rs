mod component;
mod scene_object;
mod serialize;
mod test;
mod traits;
mod resource {
    use bevy_ecs::system::Resource;
    use bevy_reflect::TypeRegistry;

    #[derive(Resource, Default)]
    pub struct SceneManager {
        pub registry: TypeRegistry,
    }
}

pub use component::{
    add_entity_to_scene, load_scene, save_scene, to_serialized_scene, unload_scene,
};

pub fn register_scene_types(world: &mut bevy_ecs::world::World) {
    world.init_resource::<resource::SceneManager>();
    world.init_component::<traits::SceneData>();
}
