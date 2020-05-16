use sfml::window::Event as SFEvent;
use std::hash::{Hash, Hasher};
use crate::gui::Widget;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum EventType {
    Input,
    CreateGui,
}

#[derive(Debug, Clone)]
pub enum EventData {
    SFMLInput(SFEvent),
    Filename(String),
}

#[derive(Debug, Clone)]
pub struct Event {
    pub t: EventType,
    pub data: EventData
}

impl Event {
    pub fn new(t: EventType, data: EventData) -> Event {
        Event {
            t: t,
            data: data
        }
    }
}

// #[derive(PartialEq)]
// struct WrappedSfEvent {
//     sf_event: SFEvent,
// }

// impl WrappedSfEvent {
//     fn new(sf_event: SFEvent) -> Self {
//         WrappedSfEvent {
//             sf_event: sf_event,
//         }
//     }
// }

// impl Hash for WrappedSfEvent {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         match self.sf_event {
//             Event::Closed => 
//             Resized {
//                 width: u32,
//                 height: u32,
//             },
//             LostFocus,
//             GainedFocus,
//             TextEntered {
//                 unicode: char,
//             },
//             KeyPressed {
//                 code: Key,
//                 alt: bool,
//                 ctrl: bool,
//                 shift: bool,
//                 system: bool,
//             },
//             KeyReleased {
//                 code: Key,
//                 alt: bool,
//                 ctrl: bool,
//                 shift: bool,
//                 system: bool,
//             },
//             MouseWheelScrolled {
//                 wheel: Wheel,
//                 delta: f32,
//                 x: i32,
//                 y: i32,
//             },
//             MouseButtonPressed {
//                 button: Button,
//                 x: i32,
//                 y: i32,
//             },
//             MouseButtonReleased {
//                 button: Button,
//                 x: i32,
//                 y: i32,
//             },
//             MouseMoved {
//                 x: i32,
//                 y: i32,
//             },
//             MouseEntered,
//             MouseLeft,
//             JoystickButtonPressed {
//                 joystickid: u32,
//                 button: u32,
//             },
//             JoystickButtonReleased {
//                 joystickid: u32,
//                 button: u32,
//             },
//             JoystickMoved {
//                 joystickid: u32,
//                 axis: Axis,
//                 position: f32,
//             },
//             JoystickConnected {
//                 joystickid: u32,
//             },
//             JoystickDisconnected {
//                 joystickid: u32,
//             },
//             TouchBegan {
//                 finger: u32,
//                 x: i32,
//                 y: i32,
//             },
//             TouchMoved {
//                 finger: u32,
//                 x: i32,
//                 y: i32,
//             },
//             TouchEnded {
//                 finger: u32,
//                 x: i32,
//                 y: i32,
//             },
//             SensorChanged {
//                 type_: Type,
//                 x: f32,
//                 y: f32,
//                 z: f32,
//             },
//     }
// }