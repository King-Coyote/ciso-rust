use crate::events::Event;
use crate::rendering::Renderer;

pub trait Widget
{
    type R: Renderer;

    fn draw(&self, dt: f32, renderer: &mut Self::R);
    fn update(&self, dt: f32);
    fn handle_event(&mut self, event_tuple: &mut (bool, Event));
    fn is_closed(&self) -> bool;
    fn close(&mut self);
}