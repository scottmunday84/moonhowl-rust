use crate::ecs::component::IComponent;
use crate::ecs::prelude::Entity;
use std::sync::atomic::{AtomicUsize, Ordering};

pub enum System {
    Registered(usize),
    Unregistered,
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
        match self {
            System::Registered(id) => entity.has_registered_component::<T>(id),
            System::Unregistered => entity.has_component::<T>(),
        }
    }

    pub fn get_component<'a, T: IComponent>(&self, entity: &'a mut Entity) -> Option<&'a T> {
        match self {
            System::Registered(id) => entity.get_registered_component::<T>(id),
            System::Unregistered => entity.get_component::<T>(),
        }
    }
}
