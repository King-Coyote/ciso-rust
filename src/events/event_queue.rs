use std::vec::Vec;
use std::collections::VecDeque;
use std::collections::HashMap;
use std::sync::{Weak, Arc, Mutex};
use crate::events::{Event, EventHandler, EventType};

type WeakMutexHandler = Weak<Mutex<EventHandler>>;

pub struct EventQueue {
    events: VecDeque<Event>,
    handlers: HashMap<EventType, Vec<WeakMutexHandler>>,
}

impl EventQueue {
    pub fn new() -> EventQueue {
        EventQueue{
            events: VecDeque::new(),
            handlers: HashMap::new()
        }
    }

    pub fn process_events(&mut self) {
        loop {
            let event = match self.events.pop_front() {
                Some(e) => e, None => break
            };
            let mut dirty = false;
            let vec = match self.handlers.get_mut(&event.t) {
                Some(v) => v,
                None => continue
            };

            for handler in vec.iter() {
                match handler.upgrade() {
                    Some(arc) => {
                        let handler: &EventHandler = &*arc.lock().unwrap();
                        handler.handle_event(event.clone());
                    },
                    None => dirty = true
                }
            }

            if dirty {
                vec.retain(|h| h.upgrade().is_some());
            }
        }
    }

    pub fn post(&mut self, e: Event) {
        println!("Event posted: {:?}", e);
        self.events.push_back(e);
    }

    pub fn register(&mut self, handler: &Arc<Mutex<EventHandler>>, e: EventType) {
        println!("Handler registered: for event type {:?}.", e);
        let handler_vec = self.handlers.entry(e).or_insert(Vec::new());
        handler_vec.push(Arc::downgrade(handler));
    }

    pub fn deregister(&mut self, handler: &Arc<Mutex<EventHandler>>, e: EventType) {
        println!("Handler de-registered: for event type {:?}.", e);
        let vec = match self.handlers.get_mut(&e) {
            Some(v) => v,
            None => return
        };
        vec.retain(|h| {
            if let Some(arc) = h.upgrade() {
                return !(*arc.lock().unwrap() == *handler.lock().unwrap());
            }
            return true;
        });
    }
}