use std::sync::{Arc, Mutex, MutexGuard};
use std::ops::Deref;
use rlua::{FromLua, Value, Context,Result, Table};
use sfml::system::Vector2f;

pub type Shared<T> = Arc<Mutex<T>>;

pub fn shared<T>(thing: T) -> Shared<T> {
    Arc::new(Mutex::new(thing))
}

pub fn table_to_pair<'lua, T: FromLua<'lua> + Send + Sync>(t: Table<'lua>) -> Result<(T, T)> {
    let a = t.get::<u32, T>(1)?;
    let b = t.get::<u32, T>(2)?;
    Ok((a, b))
}

// pub struct LuaPair<'lua, A: FromLua<'lua>, B: FromLua<'lua>>((A, B));

// impl<'lua, A: FromLua<'lua>, B: FromLua<'lua>> FromLua<'lua> for LuaPair<'lua, A, B> {
//     fn from_lua(v: Value<'lua>, _: Context<'lua>) -> Result<Self> {
//         match v {
//             Value::Table(t) => {
//                 let a: A = t.get::<A>(0);
//                 Ok((

//                 )),
//             }
//             _ => Ok(true),
//         }
//     }
// }