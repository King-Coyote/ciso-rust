use std::sync::{Arc, Mutex,};
use rlua::{FromLua, Result, Table, Value, Error as LuaError};
use crate::error::Error;
use std::env;
use std::path::{Path, PathBuf,};

pub type Shared<T> = Arc<Mutex<T>>;

pub fn shared<T>(thing: T) -> Shared<T> {
    Arc::new(Mutex::new(thing))
}

pub fn table_to_pair<'lua, T: FromLua<'lua> + Send + Sync>(t: Table<'lua>) -> Result<(T, T)> {
    let a = t.get::<u32, T>(1)?;
    let b = t.get::<u32, T>(2)?;
    Ok((a, b))
}

// spreads the contents of b into a. consumes b.
pub fn lua_spread_tables<'lua>(a: &Table<'lua>, b: Table<'lua>) -> Result<()> {
    for pair in b.pairs::<Value, Value>() {
        let (k, v) = pair?;
        a.set(k, v)?;
    }
    Ok(())
}

pub fn wrap_error_for_lua(error: Error) -> Result<()> {
    Err(LuaError::ExternalError(
        Arc::new(error)
    ))
}

pub fn get_asset_path<P: AsRef<Path>>(p: P) -> PathBuf {
    env::current_dir().unwrap()
        .join("assets")
        .join(p)
}