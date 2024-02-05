mod component;
mod scene_object;
mod serialize;
mod test;
mod traits;
mod resource {
    use bevy_ecs::{component::ComponentId, system::Resource};

    use super::traits::SerializableComponent;

    #[derive(Resource, Default, Debug)]
    pub struct SceneManager {
        pub serializable_components: Vec<ComponentId>,
    }
}
