use crate::events::*;
use sfml::window::Event as SFEvent;
use sfml::graphics::RenderWindow;
use crossbeam_channel::Sender;

pub fn process_input(window: &mut RenderWindow, event_tx: &Sender<Event>) {

}