pub mod component;
mod entity;
pub mod system;

pub mod prelude {
    pub use crate::ecs::component::IComponent;
    pub use crate::ecs::entity::{Entity, EntityCheck};
    pub use crate::ecs::system::System;
}
