use crate::ecs::system::System;
use std::any::{Any, TypeId};
use std::collections::{HashMap, HashSet};

static UNREGISTERED_SYSTEM: System = System::Unregistered;

pub trait IComponent: Any {}

pub struct Components {
    components: HashMap<TypeId, Box<dyn Any>>,
    registered_components: HashMap<TypeId, HashSet<&'static str>>,
}

impl Components {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            registered_components: HashMap::new(),
        }
    }

    pub fn has_component<T: IComponent>(&self) -> bool {
        self.components.contains_key(&TypeId::of::<T>())
    }

    pub fn has_system_component<T: IComponent>(&self, system: &System) -> bool {
        let components = &self.components;
        let registered_components = &self.registered_components;

        match system {
            System::Registered(name) => {
                if !components.contains_key(&TypeId::of::<T>()) {
                    return false;
                }

                match registered_components.get(&TypeId::of::<T>()) {
                    Some(registered_component) => !registered_component.contains(name),
                    None => true,
                }
            }
            System::Unregistered => self.has_component::<T>(),
        }
    }

    pub fn get_component<T: IComponent>(&self) -> Option<&T> {
        match self.components.get(&TypeId::of::<T>()) {
            Some(component) => (**component).downcast_ref::<T>(),
            None => None,
        }
    }

    pub fn get_system_component<T: IComponent>(&mut self, system: &System) -> Option<&T> {
        let components = &self.components;
        let registered_components = &mut self.registered_components;

        match system {
            System::Registered(name) => {
                match components.get(&TypeId::of::<T>()) {
                    Some(component) => {
                        match registered_components.get_mut(&TypeId::of::<T>()) {
                            Some(registered_component) => registered_component.insert(name),
                            None => false, // Will never happen
                        };

                        (**component).downcast_ref::<T>()
                    }
                    None => None,
                }
            }
            System::Unregistered => self.get_component::<T>(),
        }
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

    pub fn check<F>(&self, check_fnc: F) -> ComponentCheck
    where
        F: FnOnce(&Components) -> bool,
    {
        match check_fnc(&self) {
            true => ComponentCheck::Valid,
            false => ComponentCheck::Invalid,
        }
    }
}

pub enum ComponentCheck {
    Valid,
    Invalid,
}

impl ComponentCheck {
    pub fn then<F>(&self, then_fnc: F)
    where
        F: FnOnce(),
    {
        match self {
            ComponentCheck::Valid => then_fnc(),
            ComponentCheck::Invalid => (),
        }
    }
}

#[macro_export]
macro_rules! has_component {
    ($components:expr, $comp:ty) => {
        $components.has_component::<$comp>()
    };
    ($components:expr, $comp:ty, $system:expr) => {
        $components.has_system_component::<$comp>($system)
    };
}

#[macro_export]
macro_rules! get_component {
    ($components:expr, $comp:ty) => {
        $components.get_component::<$comp>()
    };
    ($components:expr, $comp:ty, $system:expr) => {
        $components.get_system_component::<$comp>($system)
    };
}
