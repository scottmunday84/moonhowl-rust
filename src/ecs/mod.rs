pub mod component;
mod entity;
pub mod system;

pub mod prelude {
    pub use crate::ecs::component::{ComponentCheck, IComponent};
    pub use crate::ecs::entity::Entity;
    pub use crate::ecs::system::System;
}
