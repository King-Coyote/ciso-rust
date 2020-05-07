use crate::events::{Event, EventType};
use std::collections::HashMap;

type Callback = fn(Event) -> ();

#[derive(Eq, PartialEq)]
pub struct EventHandler {
    handle_fns: HashMap<EventType, Callback>
}

impl EventHandler {
    pub fn new() -> EventHandler {
        EventHandler {
            handle_fns: HashMap::new()
        }
    }

    pub fn bind(&mut self, t: EventType, f: Callback) -> &mut EventHandler {
        self.handle_fns.insert(t, f);
        return self;
    }

    pub fn handle_event(&self, event: Event) {
        if let Some(f) = self.handle_fns.get(&event.t) {
            f(event);
        }
    }
}