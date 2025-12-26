use std::any::Any;

pub trait IComponent: Any {
    fn as_any(&self) -> &dyn Any;
}
