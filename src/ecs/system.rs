use crate::ecs::component::IComponent;
use crate::ecs::prelude::Entity;
use std::sync::atomic::{AtomicUsize, Ordering};

pub enum System {
    Registered(usize),
}

impl System {
    pub fn new() -> Self {
        System::Registered(Self::get_id())
    }

    fn get_id() -> usize {
        static COUNTER: AtomicUsize = AtomicUsize::new(1);
        COUNTER.fetch_add(1, Ordering::Relaxed)
    }

    pub fn has_component<T: IComponent>(&self, entity: &Entity) -> bool {
        if !entity.has_component::<T>() {
            return false;
        }

        match self {
            System::Registered(id) => entity.has_registered_component::<T>(id),
        }
    }

    pub fn get_component<'a, T: IComponent>(&self, entity: &'a mut Entity) -> Option<&'a T> {
        if !entity.has_component::<T>() {
            return None;
        }

        match self {
            System::Registered(id) => entity.get_registered_component::<T>(id),
        }
    }
}
