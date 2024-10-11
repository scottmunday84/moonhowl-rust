mod component;
mod system;

pub(crate) mod preamble {
    use crate::ecs::component;
    use crate::ecs::system;

    pub use component::{ComponentCheck, Components, IComponent};
    pub use system::System;
}