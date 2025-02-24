use std::any::Any;

pub trait IComponent: Any {
    fn as_any(&self) -> &dyn Any;
}

impl<T: Any> IComponent for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
