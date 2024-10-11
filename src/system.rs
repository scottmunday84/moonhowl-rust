use crate::component::{ComponentCheck, Components};

pub enum System {
    Registered(&'static str),
    Unregistered
}
