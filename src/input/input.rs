use crate::events::*;
use sfml::window::Event as SFEvent;
use sfml::graphics::RenderWindow;
use crossbeam_channel::Sender;

pub fn process_input(window: &mut RenderWindow, event_tx: &Sender<Event>) {
    while let Some(event) = window.poll_event() {
        match event {
            SFEvent::Closed => window.close(),
            _ => event_tx.send(Event::Input(event)).expect("whoops, event not sent from input")
        };
    }
}