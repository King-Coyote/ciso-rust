use std::vec::Vec;

pub struct EcsStorage<T> 
{
    components: T,
}

impl<T> EcsStorage<T> 
    where T: IntoIterator + Default
{
    pub fn new() -> EcsStorage<T> {
        EcsStorage::<T> {
            components: T::default(),
        }
    }
}