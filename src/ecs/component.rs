use std::any::Any;

pub trait IComponent: Any {}

pub enum ComponentCheck {
    Valid,
    Invalid,
}

impl ComponentCheck {
    pub fn then<F>(&self, then_fnc: F)
    where
        F: FnOnce(),
    {
        match self {
            ComponentCheck::Valid => then_fnc(),
            ComponentCheck::Invalid => (),
        }
    }
}
