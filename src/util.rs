use std::sync::{Arc, Mutex,};
use rlua::{FromLua, Result, Table};

pub type Shared<T> = Arc<Mutex<T>>;

pub fn shared<T>(thing: T) -> Shared<T> {
    Arc::new(Mutex::new(thing))
}

pub fn table_to_pair<'lua, T: FromLua<'lua> + Send + Sync>(t: Table<'lua>) -> Result<(T, T)> {
    let a = t.get::<u32, T>(1)?;
    let b = t.get::<u32, T>(2)?;
    Ok((a, b))
}