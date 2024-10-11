use std::any::{Any, TypeId};
use std::collections::{HashMap, HashSet};
use crate::system::System;

static UNREGISTERED_SYSTEM: System = System::Unregistered;

pub trait IComponent: Any {}

pub struct Components {
    components: HashMap<TypeId, Box<dyn Any>>,
    registered_components: HashMap<TypeId, HashSet<&'static str>>
}

impl Components  {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            registered_components: HashMap::new()
        }
    }

    pub fn has_unregistered_component<T: IComponent>(&self) -> bool {
        self.has_component::<T>(&UNREGISTERED_SYSTEM)
    }

    pub fn has_component<T: IComponent>(&self, system: &System) -> bool {
        let components = &self.components;
        let registered_components = &self.registered_components;

        match system {
            System::Registered(name) => {
                if !components.contains_key(&TypeId::of::<T>()) {
                    return false;
                }

                match registered_components.get(&TypeId::of::<T>()) {
                    Some(registered_component) => !registered_component.contains(name),
                    None => true
                }
            }
            System::Unregistered => components.contains_key(&TypeId::of::<T>())
        }
    }

    pub fn get_unregistered_component<T: IComponent>(&mut self) -> Option<&T> {
        self.get_component(&UNREGISTERED_SYSTEM)
    }

    pub fn get_component<T: IComponent>(&mut self, system: &System) -> Option<&T> {
        let components = &self.components;
        let registered_components = &mut self.registered_components;

        match system {
            System::Registered(name) => {
                match components.get(&TypeId::of::<T>()) {
                    Some(component) => {
                        match registered_components.get_mut(&TypeId::of::<T>()) {
                            Some(registered_component) => registered_component.insert(name),
                            None => false  // Should never reach
                        };

                        (**component).downcast_ref::<T>()
                    },
                    None => None
                }
            },
            System::Unregistered => {
                match components.get(&TypeId::of::<T>()) {
                    Some(component) => (**component).downcast_ref::<T>(),
                    None => None
                }
            }
        }
    }

    pub fn add_component<T: IComponent>(&mut self, component: T) -> &mut Self {
        let boxed_component = Box::new(component);
        self.components.insert(TypeId::of::<T>(), boxed_component);
        self.registered_components.insert(TypeId::of::<T>(), HashSet::new());

        self
    }

    pub fn remove_component<T: IComponent>(&mut self) -> &mut Self {
        self.components.remove(&TypeId::of::<T>());
        self.registered_components.remove(&TypeId::of::<T>());

        self
    }

    pub fn check<F>(&self, test_fnc: F) -> ComponentCheck where F: FnOnce(&Components) -> bool {
        match test_fnc(&self) {
            true => ComponentCheck::Valid,
            false => ComponentCheck::Invalid
        }
    }
}

pub enum ComponentCheck {
    Valid,
    Invalid
}

impl ComponentCheck {
    pub fn then_run_system<F>(&self, run_fnc: F) where F: FnOnce() {
        match self {
            ComponentCheck::Valid => run_fnc(),
            ComponentCheck::Invalid => ()
        }
    }
}
