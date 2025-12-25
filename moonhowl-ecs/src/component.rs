use std::any::Any;

pub trait IComponent: Any {
    fn as_any(&self) -> &dyn Any;
}

#[macro_export]
macro_rules! ecs_component {
    ($t:ty) => {
        impl IComponent for $t {
            fn as_any(&self) -> &dyn Any {
                self
            }
        }
    };
}
