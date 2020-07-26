use std::vec::Vec;
use std::collections::VecDeque;
use std::collections::HashMap;
use std::sync::{Weak, Arc, Mutex,};
use std::iter::Iterator;
use std::mem::swap;
use crate::events::{Event, EventType};
use rlua::{UserData, UserDataMethods};
use crossbeam_channel::{Sender, Receiver, unbounded,};

pub struct EventQueue {
    events_current: VecDeque<Event>,
    events_next: VecDeque<Event>,
    inbound: Receiver<Event>,
    outbound_tx: Sender<Event>,
}

impl EventQueue {
    pub fn new() -> (Sender<Event>, Receiver<Event>, EventQueue) {
        let (inbound_tx, inbound_rx) = unbounded();
        let (outbound_tx_, outbound_rx_) = unbounded();
        let event_queue = EventQueue{
            events_current: VecDeque::new(),
            events_next: VecDeque::new(),
            inbound: inbound_rx,
            outbound_tx: outbound_tx_,
        };
        // give channel that can send this queue events to post
        (inbound_tx, outbound_rx_, event_queue)
    }

    pub fn post(&mut self, e: Event) {
        self.events_next.push_back(e);
    }

    pub fn poll(&mut self) -> impl Iterator<Item = &mut Event> {
        return self.events_current.iter_mut();
    }

    pub fn new_frame(&mut self) {
        if self.events_next.len() == 0 && self.events_current.len() == 0 {
            return;
        }
        swap(&mut self.events_next, &mut self.events_current);
        self.events_next.clear();
    }

}