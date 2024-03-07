use std::{hash::Hash, sync::atomic::AtomicUsize};

use bevy_ecs::world::World;

#[derive(Debug, Eq, Clone, Copy)]
pub struct AssetID {
    pub(crate) id: usize,
    pub(crate) unload_condition: Option<fn(&Self, world: &World) -> bool>,
}

impl PartialEq for AssetID {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for AssetID {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl AssetID {
    pub fn get_id() -> usize {
        pub(crate) static COUNTER: AtomicUsize = AtomicUsize::new(1);
        COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    }
}
