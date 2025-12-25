pub mod component;
mod entity;
pub mod system;

pub mod prelude {
    pub use crate::component::IComponent;
    pub use crate::entity::{Entity, EntityCheck, EntitySystem};
    pub use crate::system::System;
    pub use crate::ecs_component;
}
