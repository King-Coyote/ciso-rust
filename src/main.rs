mod events;
mod input;
mod rendering;
mod resources;
use events::*;
use std::sync::{Arc, Mutex};
use input::process_input;
use rendering::*;
use resources::*;
use sfml::window::Style;
use std::path::Path;
use sfml::graphics::{RenderWindow, Texture, Sprite};

extern crate sfml;

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
    // let handler = Arc::new(Mutex::new(EventHandler::new()));
    // &*handler.lock().unwrap()
    //     .bind(EventType::Test, |e| println!("Event handled: {:?}", e))
    //     .bind(EventType::Input, |e| println!("Input event handled: {:?}", e));
    // queue.register(&handler, EventType::Test);
    // queue.post(Event::new(EventType::Test, EventData::Test("fuck".to_string())));
    // queue.post(Event::new(EventType::Input, EventData::Mouse(0)));
    // queue.process_events();
    let tex_arc = resource_manager.get_texture("elminster_single.png");
    let mut sprite = Sprite::new();
    sprite.set_texture(&**tex_arc, false);
    println!("All systems initialized.");
    while window.is_open() {
        process_input(&mut window, &mut queue);
        queue.process_events();
        renderer.begin(&mut window);
        renderer.draw_sprite(&mut window, &sprite);
        renderer.end(&mut window);
    }
    
}
