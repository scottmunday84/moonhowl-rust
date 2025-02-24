use crate::ecs::prelude::IComponent;
use std::any::{Any, TypeId};
use std::collections::{HashMap, HashSet};

pub struct Entity {
    components: HashMap<TypeId, Box<dyn Any>>,
    registered_components: HashMap<TypeId, HashSet<usize>>,
}

impl Entity {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            registered_components: HashMap::new(),
        }
    }

    pub fn has_component<T: IComponent>(&self) -> bool {
        self.components.contains_key(&TypeId::of::<T>())
    }

    pub fn has_registered_component<T: IComponent>(&self, id: &usize) -> bool {
        if !self.has_component::<T>() {
            return false;
        }

        if let Some(registered_component) = self.registered_components.get(&TypeId::of::<T>()) {
            return !registered_component.contains(id);
        }

        true
    }

    pub fn get_component<T: IComponent>(&self) -> Option<&T> {
        match self.components.get(&TypeId::of::<T>()) {
            Some(component) => (**component).downcast_ref::<T>(),
            None => None,
        }
    }

    pub fn get_registered_component<T: IComponent>(&mut self, id: &usize) -> Option<&T> {
        if !self.has_component::<T>() {
            return None;
        }

        if let Some(registered_component) = self.registered_components.get_mut(&TypeId::of::<T>()) {
            registered_component.insert(*id);
        };

        self.get_component::<T>()
    }

    pub fn add_component<T: IComponent>(&mut self, component: T) -> &mut Self {
        let boxed_component = Box::new(component);
        self.components.insert(TypeId::of::<T>(), boxed_component);
        self.registered_components
            .insert(TypeId::of::<T>(), HashSet::new());

        self
    }

    pub fn drop_component<T: IComponent>(&mut self) -> &mut Self {
        self.components.remove(&TypeId::of::<T>());
        self.registered_components.remove(&TypeId::of::<T>());

        self
    }

    pub fn check<F>(&self, check_fnc: F) -> EntityCheck
    where
        F: FnOnce(&Entity) -> bool,
    {
        match check_fnc(&self) {
            true => EntityCheck::On,
            false => EntityCheck::Off,
        }
    }
}

pub enum EntityCheck {
    On,
    Off,
}

impl EntityCheck {
    pub fn then<F>(&self, then_fnc: F)
    where
        F: FnOnce(),
    {
        match self {
            EntityCheck::On => then_fnc(),
            EntityCheck::Off => (),
        }
    }
}
