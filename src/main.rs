mod events;
mod input;
mod rendering;
mod resources;
mod gui;
mod game;

extern crate sfml;
extern crate rlua;

use std::time::{Duration, Instant};
use events::{EventQueue, Event};
use input::process_input;
use rendering::{SimpleWindow, Renderer};
use resources::ResourceManager;
use game::Game;
use gui::Gui;
use sfml::window::Style;
use sfml::graphics::RenderWindow;

fn main() {
    let mut event_queue = EventQueue::new();
    let mut window = SimpleWindow::new(
        RenderWindow::new(
            (800,600),
            "Ciso",
            Style::CLOSE,
            &Default::default(),
        )
    );
    let mut resource_manager = ResourceManager::new();
    let mut gui = Gui::new(&mut event_queue);
    let mut game = Game::new();

    println!("All systems initialized.");
    let mut elapsed = Instant::now();
    
    while window.is_open() {
        let dt = Instant::now().duration_since(elapsed).as_secs_f32();
        elapsed = Instant::now();
        // get input from window
        window.process_input(&mut event_queue);
        // Do all updates
        gui.update(dt, &mut event_queue);
        game.update(dt, &mut event_queue);
        // Render to the window
        window.begin();
        gui.draw(dt, &mut window);
        game.draw(dt, &mut window);
        window.end();

        event_queue.new_frame();
    }
    
}
