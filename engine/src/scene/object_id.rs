use std;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicUsize, Ordering};

use bevy_reflect::Reflect;

#[derive(Debug, Eq, Clone, Copy, PartialOrd, Reflect)]
pub struct ObjectID {
    pub(crate) id: usize,
}

impl PartialEq for ObjectID {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for ObjectID {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

pub(crate) static COUNTER: AtomicUsize = AtomicUsize::new(1);
pub(crate) static ACTION_COUNTER: AtomicUsize = AtomicUsize::new(1);
pub(crate) static SCENE_COUNTER: AtomicUsize = AtomicUsize::new(1);

#[allow(unused)]
impl ObjectID {
    pub fn get_id() -> usize {
        COUNTER.fetch_add(1, Ordering::Relaxed)
    }

    pub fn get_id_from_counter(counter: CounterType) -> usize {
        match counter {
            CounterType::Global => COUNTER.fetch_add(1, Ordering::Relaxed),
            CounterType::Actions => ACTION_COUNTER.fetch_add(1, Ordering::Relaxed),
            CounterType::Scenes => SCENE_COUNTER.fetch_add(1, Ordering::Relaxed),
        }
    }

    pub fn new(counter: CounterType) -> ObjectID {
        ObjectID {
            id: Self::get_id_from_counter(counter),
        }
    }
}

pub enum CounterType {
    Global,
    Actions,
    Scenes,
}
