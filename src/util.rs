use std::sync::{Arc, Mutex, MutexGuard};
use std::ops::Deref;

pub type Shared<T> = Arc<Mutex<T>>;

pub fn shared<T>(thing: T) -> Shared<T> {
    Arc::new(Mutex::new(thing))
}