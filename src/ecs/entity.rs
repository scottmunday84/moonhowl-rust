use crate::ecs::prelude::{IComponent, System};
use std::any::TypeId;
use std::collections::{HashMap, HashSet};

pub struct Entity {
    components: HashMap<TypeId, Box<dyn IComponent>>,
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
            Some(component) => (**component).as_any().downcast_ref::<T>(),
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
        if let EntityCheck::On = self {
            then_fnc();
        }
    }
}

pub enum EntitySystem<'a, 'b> {
    Reader(&'a Entity, &'b System),
    Writer(&'a mut Entity, &'b System),
}

impl <'a, 'b> EntitySystem<'a, 'b> {
    pub fn new_reader(entity: &'a Entity, system: &'b System) -> Self {
        Self::Reader(entity, system)
    }
    
    pub fn new_writer(entity: &'a mut Entity, system: &'b System) -> Self {
        Self::Writer(entity, system)
    }

    pub fn has_component<T: IComponent>(&self) -> bool {
        match self {
            EntitySystem::Reader(entity, system) => system.has_component::<T>(entity),
            EntitySystem::Writer(entity, system) => system.has_component::<T>(entity),
        }
    }
    
    pub fn get_component<T: IComponent>(&mut self) -> Option<&T> {
        if let EntitySystem::Writer(entity, system) = self {
            return system.get_component::<T>(entity);
        }
        
        None
    }
}
