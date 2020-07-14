use crate::rendering::Renderer;
use sfml::graphics::{
    Color, RenderTarget, RenderWindow,
    Shape, Text, Sprite,
};
use sfml::window::Event as SFEvent;
use crate::events::*;

/// A window that simply draws to an sfml window without any batching etc
pub struct SimpleWindow {
    window: RenderWindow,
}

impl SimpleWindow {
    pub fn new(window: RenderWindow) -> Self {
        SimpleWindow {
            window: window
        }
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }

    pub fn process_input(&mut self, event_queue: &mut EventQueue) {
        while let Some(event) = self.window.poll_event() {
            match event {
                SFEvent::Closed => self.window.close(),
                _ => event_queue.post(Event::new(EventType::Input, EventData::SFMLInput(event))),
            };
        }
    }
}

impl Renderer for SimpleWindow {
    // call this when drawing begins
    fn begin(&mut self) {
        self.window.clear(Color::BLACK);
    }

    fn draw_shape<'a>(&mut self, drawable: &impl Shape<'a>) {
        self.window.draw(drawable);
    }

    fn draw_sprite(&mut self, drawable: &Sprite) {
        self.window.draw(drawable);
    }

    fn draw_text(&mut self, drawable: &Text) {
        self.window.draw(drawable);
    }

    // call this when drawing should end
    fn end(&mut self) {
        self.window.display();
    }
}