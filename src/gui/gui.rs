use crate::events::*;
use crate::gui::Widget;
use crate::gui::Panel;
use crate::rendering::*;
use crate::util::*;
use crate::resources::ResourceManager;
use crate::scripting::{Scripting, LuaChannel,};
use std::sync::{Arc, Mutex};
use rlua::Table;
use rlua::Result;
use crossbeam_channel::Receiver;
use sfml::window::Event as SFEvent;

pub struct Gui<T: Renderer + 'static> {
    event_rx: Receiver<Event>,
    root_widgets: Vec<Box<dyn Widget<R = T>>>,
    resource_manager: Shared<ResourceManager>,
}

impl<T: Renderer> Gui<T> {
    pub fn new(
        event_rx: Receiver<Event>,
        resource_manager: Shared<ResourceManager>
    ) -> Self {
        let mut gui = Gui {
            event_rx: event_rx,
            root_widgets: vec!(),
            resource_manager: resource_manager
        };
        return gui;
    }

    pub fn update(&mut self, dt: f32) {
        loop {
            match self.event_rx.try_recv() {
                Ok(event) => {
                    self.handle_event(event);
                },
                _ => break
            }
        }
    }

    pub fn draw(&mut self, dt: f32, renderer: &mut T) {
        self.root_widgets.retain(|w| {
            w.update(dt);
            w.draw(dt, renderer);
            !w.is_closed()
        });
    }

    fn add_widget(&mut self, filename: &'static str) {
        self.root_widgets.push(Box::new(Panel::new((100.0, 100.0), (0.0,0.0))));
    }

    fn handle_event(&mut self, e: Event) {
        println!("Handling input event!");
        // match e {
        //     Event::CreateGui(data_to_go_here) => self.handle_event_create(data_to_go_here),
        //     _ => {
        //         let mut event_tuple = (false, e);
        //         for widget in self.root_widgets.iter_mut() {
        //             widget.handle_event(&mut event_tuple);
        //         }
        //     }
        // }
    }

    fn handle_event_create(&mut self, wat_do_here: String) {
        
    }
}