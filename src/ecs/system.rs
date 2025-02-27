use crate::ecs::component::IComponent;
use crate::ecs::prelude::Entity;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct System(usize);

impl System {
    pub fn new() -> Self {
        Self(Self::get_id())
    }

    fn get_id() -> usize {
        static COUNTER: AtomicUsize = AtomicUsize::new(1);
        COUNTER.fetch_add(1, Ordering::Relaxed)
    }

    pub fn has_component<T: IComponent>(&self, entity: &Entity) -> bool {
        entity.has_registered_component::<T>(&self.0)
    }

    pub fn get_component<'a, T: IComponent>(&self, entity: &'a mut Entity) -> Option<&'a T> {
        entity.get_registered_component::<T>(&self.0)
    }
}
