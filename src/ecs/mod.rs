pub mod component;
pub mod system;

pub mod preamble {
    pub use crate::ecs::component::{ComponentCheck, Components, IComponent};
    pub use crate::ecs::system::System;
}
