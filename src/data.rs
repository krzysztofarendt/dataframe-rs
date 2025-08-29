use std::any::Any;

pub trait Data: Any {
    fn as_any(&self) -> &dyn Any;
}

impl<T: Any> Data for T {
    fn as_any(&self) -> &dyn Any { self }
}
