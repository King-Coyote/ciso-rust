use crate::events::*;
use crate::rendering::*;
use crate::game::*;
use crate::util::*;
use crate::resources::ResourceManager;
use crate::scripting::{Scripting, LuaChannel};
use crossbeam_channel::Receiver;
use rlua::prelude::*;
use rlua::Result;

const NUM_ENT: u32 = 10;
/// Represents the game as an ECS system
pub struct Game<'a> {
    event_rx: Receiver<Event>,
    resource_manager: Shared<ResourceManager>,

    positions: Vec<Position>,
    appearances: Vec<Appearance<'a>>,
    movements: Vec<Movement>,

    sys_movement: MovementSystem,
    sys_appearance: AppearanceSystem,
}

impl<'a> Game<'a> {
    pub fn new(
        event_rx: Receiver<Event>,
        resource_manager: Shared<ResourceManager>
    ) -> Self {
        Game {
            event_rx: event_rx,
            resource_manager: resource_manager,

            positions: vec![],
            appearances: vec![],
            movements: vec![],

            sys_movement: Default::default(),
            sys_appearance: Default::default(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.sys_movement.update(dt, &self.movements, &mut self.positions);
        self.sys_appearance.update(dt, &self.appearances);
    }

    pub fn draw(&self, dt: f32, renderer: &mut impl Renderer) {
        self.sys_appearance.draw(dt, &self.appearances, renderer);
    }
}