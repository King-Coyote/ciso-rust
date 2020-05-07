use sfml::window::Event as SFEvent;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum EventType {
    Input,
    Test,
}

#[derive(Debug, Clone)]
pub enum EventData {
    Test(String),
    SFMLInput(SFEvent),
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

// impl Display for EventType {
//     fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
//         match *self {
//             EventType::Input => f.write_str("Input"),
//             EventType::Test => f.write_str("Test"),
//         }
//     }
// }

// impl Display for EventData {
//     fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
//         match *self {
//             EventData::Test(ref inner) => {
//                 f.write_str("Test").unwrap();
//                 return ::std::fmt::Display::fmt(inner, f);
//             },
//         }
//     }
// }

// impl Display for Event {
//     fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        
//     }
// }