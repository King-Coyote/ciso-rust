use sfml::graphics::{Sprite, Texture,};
use std::sync::Arc;

pub trait Component {
    fn get_level(&self) -> u32;
    fn instantiate(&self) {}
    fn destroy(&self) {}
}

pub struct Position {
    level: u32,
    x: i32,
    y: i32,
}

impl Component for Position {
    fn get_level(&self) -> u32 {
        self.level
    }
}

pub struct Movement {
    level: u32,
}

impl Component for Movement {
    fn get_level(&self) -> u32 {
        self.level
    }
}

pub struct Appearance<'a> {
    level: u32,
    texture_resource: Arc<Texture>,
    sprite: Sprite<'a>,
}

impl<'a> Component for Appearance<'a> {
    fn get_level(&self) -> u32 {
        self.level
    }
}