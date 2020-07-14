use sfml::graphics::{Shape, Text, Sprite};

pub trait Renderer {
    // call this when drawing begins
    fn begin(&mut self);
    fn draw_shape<'a>(&mut self, drawable: &impl Shape<'a>);
    fn draw_sprite(&mut self, drawable: &Sprite);
    fn draw_text(&mut self, drawable: &Text);
    // call this when drawing should end
    fn end(&mut self);
}