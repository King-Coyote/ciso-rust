mod events;
mod input;
mod rendering;
mod resources;
mod gui;

extern crate sfml;
extern crate rlua;

use std::time::{Duration, Instant};
use events::{EventQueue, Event};
use input::process_input;
use rendering::{SimpleRenderer, Renderer};
use resources::ResourceManager;
use gui::Gui;
use sfml::window::Style;
use sfml::graphics::RenderWindow;

fn main() {

    let mut window = RenderWindow::new(
        (800,600),
        "Ciso",
        Style::CLOSE,
        &Default::default(),
    );

    let mut queue = EventQueue::new();
    let mut renderer = SimpleRenderer::new();
    let mut resource_manager = ResourceManager::new();
    let mut gui = Gui::new(&mut queue);

    println!("All systems initialized.");
    let mut elapsed = Instant::now();
    
    while window.is_open() {
        let dt = Instant::now().duration_since(elapsed).as_secs_f32();
        elapsed = Instant::now();

        process_input(&mut window, &mut queue);
        queue.process_events();

        renderer.begin(&mut window);
        gui.draw(dt, &mut window);
        renderer.end(&mut window);
    }
    
}
