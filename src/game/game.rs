use crate::events::*;
use crate::rendering::*;
use crate::util::*;
use crate::resources::ResourceManager;
use crate::scripting::{Scripting,};
use crossbeam_channel::Receiver;

/// Represents the game as an ECS system
pub struct Game {
    _scripting: Shared<Scripting>,
    _event_rx: Receiver<Event>,
    _resource_manager: Shared<ResourceManager>
}

impl Game {
    pub fn new(
        scripting: Shared<Scripting>,
        event_rx: Receiver<Event>,
        resource_manager: Shared<ResourceManager>
    ) -> Self {
        Game {
            _scripting: scripting,
            _event_rx: event_rx,
            _resource_manager: resource_manager,
        }
    }

    pub fn update(&mut self, _dt: f32) {

    }

    pub fn draw(&self, _dt: f32, _renderer: &mut Renderer) {
    }
}