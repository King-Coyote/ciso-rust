use crate::events::{Event, EventType};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};


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

    pub fn new_managed() -> Arc<Mutex<EventHandler>> {
        Arc::new(Mutex::new(EventHandler::new()))
    }

    pub fn bind(&mut self, t: EventType, f: impl Fn(Event) -> ()) -> &mut EventHandler {
        self.handle_fns.insert(t, f);
        return self;
    }

    pub fn handle_event(&self, event: Event) {
        if let Some(f) = self.handle_fns.get(&event.t) {
            f(event);
        }
    }
}