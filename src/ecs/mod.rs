mod component;
mod system;

pub mod preamble {
    use crate::ecs::component;
    use crate::ecs::system;

    pub use component::{ComponentCheck, Components, IComponent};
    pub use system::System;
}