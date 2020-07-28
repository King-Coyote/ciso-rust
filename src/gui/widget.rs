use crate::events::Event;
use crate::rendering::Renderer;
use sfml::window::Event as SFEvent;

pub trait Widget
{
    type R: Renderer;

    fn draw(&self, dt: f32, renderer: &mut Self::R);
    fn update(&self, dt: f32);
    fn handle_input(&mut self, handled: &mut bool, sf_event: &SFEvent);
    fn is_closed(&self) -> bool;
    fn close(&mut self);
}