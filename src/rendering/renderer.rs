use sfml::graphics::{
    Color, RenderTarget, RenderWindow,
    Shape, Text, Sprite,
};
use sfml::window::Event as SFEvent;
use crate::events::*;
use crossbeam_channel::Sender;

/// A window that simply draws to an sfml window
/// currently does not perform batching
pub struct Renderer {
    event_tx: Sender<Event>,
    window: RenderWindow,
}

impl Renderer {
    pub fn new(window: RenderWindow, event_tx: Sender<Event>) -> Self {
        Renderer {
            event_tx: event_tx,
            window: window
        }
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }

    pub fn process_input(&mut self) {
        while let Some(event) = self.window.poll_event() {
            match event {
                SFEvent::Closed => self.window.close(),
                _ => self.event_tx.send(Event::Input(event)).expect("whoops, event not sent from window")
            };
        }
    }

    pub fn begin(&mut self) {
        self.window.clear(Color::BLACK);
    }

    pub fn draw_shape<'a>(&mut self, drawable: &impl Shape<'a>) {
        self.window.draw(drawable);
    }

    pub fn draw_sprite(&mut self, drawable: &Sprite) {
        self.window.draw(drawable);
    }

    pub fn draw_text(&mut self, drawable: &Text) {
        self.window.draw(drawable);
    }

    // call this when drawing should end
    pub fn end(&mut self) {
        self.window.display();
    }
}