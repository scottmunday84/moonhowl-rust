mod component;
mod system;

pub mod preamble {
    pub use crate::component::{ComponentCheck, Components, IComponent};
    pub use crate::system::System;
}
