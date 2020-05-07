use crate::events::*;
use sfml::window::Event as SFEvent;
use sfml::graphics::RenderWindow;

pub fn process_input(window: &mut RenderWindow, event_queue: &mut EventQueue) {
    //dt: f32, q: &EventQueue, window: &RenderWindow) {
    while let Some(event) = window.poll_event() {
        match event {
            SFEvent::Closed => window.close(),
            _ => event_queue.post(Event::new(EventType::Input, EventData::SFMLInput(event))),
        };
    }
}