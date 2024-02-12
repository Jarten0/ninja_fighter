mod component;
mod scene_object;
mod serialize;
mod test;
mod traits;
mod resource {
    use bevy_ecs::{component::ComponentId, system::Resource};
    use bevy_reflect::TypeRegistry;

    use super::traits::SerializableComponent;

    #[derive(Resource, Default)]
    pub struct SceneManager {
        pub registry: TypeRegistry,
    }
}

pub fn register(world: &mut bevy_ecs::world::World) {
    world.init_resource::<resource::SceneManager>();
    world.init_component::<traits::SceneData>();
}
