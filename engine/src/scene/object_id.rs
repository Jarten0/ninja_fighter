use bevy_reflect::Reflect;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicUsize, Ordering};

pub(crate) static COUNTER: AtomicUsize = AtomicUsize::new(1);
pub(crate) static ACTION_COUNTER: AtomicUsize = AtomicUsize::new(1);
pub(crate) static SCENE_COUNTER: AtomicUsize = AtomicUsize::new(1);
pub(crate) static CLICK_COUNTER: AtomicUsize = AtomicUsize::new(1);

#[derive(Debug)]
pub struct Counter(AtomicUsize);

impl Counter {
    pub fn get(&self) -> usize {
        self.0.fetch_add(1, Ordering::Relaxed)
    }

    pub const fn new() -> Self {
        Self(AtomicUsize::new(1))
    }
}

pub trait IDCounter
where
    Self: Sized,
{
    fn get_new() -> Self;
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Reflect)]
/// A unique identifier to an instance of a component.
///
/// This is added to [`super::SceneData`] whenever an entity is registered into a scene,
/// and only if that component implements [`super::TestSuperTrait`], which allows it to
/// be picked up by an iterator.
pub struct ComponentInstanceID(usize);

impl IDCounter for ComponentInstanceID {
    fn get_new() -> ComponentInstanceID {
        pub static STATIC_ID_COUNTER: Counter = Counter::new();
        ComponentInstanceID(STATIC_ID_COUNTER.get())
    }
}

#[derive(Debug, Eq, Clone, Copy, PartialOrd, Reflect)]
// #[reflect_value]
pub struct ObjectID {
    #[reflect(ignore)]
    pub(crate) id: usize,
    counter: CounterType,
}

impl Default for ObjectID {
    fn default() -> Self {
        Self::new(CounterType::Scenes)
    }
}

impl PartialEq for ObjectID {
    /// For two identifiers to match, their number must match, and their counter must match aswell.
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.counter == other.counter
    }
}
impl core::fmt::Display for ObjectID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id.to_string())?;
        Ok(())
    }
}

impl Hash for ObjectID {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.counter.hash(state);
        self.id.hash(state);
    }
}

impl ObjectID {
    pub fn get_id() -> usize {
        COUNTER.fetch_add(1, Ordering::Relaxed)
    }

    pub fn get_id_from_counter(counter: CounterType) -> usize {
        match counter {
            CounterType::Global => COUNTER.fetch_add(1, Ordering::Relaxed),
            CounterType::Actions => ACTION_COUNTER.fetch_add(1, Ordering::Relaxed),
            CounterType::Scenes => SCENE_COUNTER.fetch_add(1, Ordering::Relaxed),
            CounterType::Component => CLICK_COUNTER.fetch_add(1, Ordering::Relaxed),
            #[allow(unreachable_patterns)]
            _ => panic!("Counter not implemented: {:?}", counter),
        }
    }

    pub fn new(counter: CounterType) -> ObjectID {
        ObjectID {
            id: Self::get_id_from_counter(counter),
            counter,
        }
    }
}

/// Before adding any variants to this, make sure you update [`ObjectID::get_id_from_counter()`]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash, Reflect)]
pub enum CounterType {
    Global,
    Actions,
    Scenes,
    Component,
}
