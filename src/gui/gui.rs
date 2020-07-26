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

pub struct Gui<T: Renderer + 'static> {
    root_widgets: Vec<Box<dyn Widget<R = T>>>,
    resource_manager: Shared<ResourceManager>,
}

impl<T: Renderer> Gui<T> {
    pub fn new(
        resource_manager: Shared<ResourceManager>
    ) -> Self {
        let mut gui = Gui {
            root_widgets: vec!(),
            resource_manager: resource_manager
        };
        return gui;
    }

    pub fn update(&mut self, dt: f32, event_queue: &mut EventQueue) {
        for event in event_queue.poll() {
            match event.t {
                EventType::CreateGui => self.handle_event_create(event),
                EventType::Input => self.handle_event_input(event),
                _ => {}
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

    fn handle_event_input(&mut self, event: &mut Event) {
        for widget in self.root_widgets.iter_mut() {
            widget.handle_event(event);
        }
    }

    fn handle_event_create(&mut self, event: &mut Event) {
        self.add_widget("");
    }
}