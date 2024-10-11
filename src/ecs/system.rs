use crate::ecs::component::{ComponentCheck, Components};

pub enum System {
    Registered(&'static str),
    Unregistered
}
