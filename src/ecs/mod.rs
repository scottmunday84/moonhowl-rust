pub mod component;
mod entity;
pub mod system;

pub mod prelude {
    pub use crate::ecs_component;
    pub use crate::ecs::component::IComponent;
    pub use crate::ecs::entity::{Entity, EntityCheck, EntitySystem};
    pub use crate::ecs::system::System;
}
