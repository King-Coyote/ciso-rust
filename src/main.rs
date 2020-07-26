mod events;
mod input;
mod rendering;
mod resources;
mod gui;
mod game;
mod scripting;
mod util;

extern crate sfml;
extern crate rlua;

use std::{
    sync::{Arc, Mutex,},
    time::{Instant,}
};
use events::{EventQueue,Event,};
use rendering::{SimpleWindow, Renderer};
use resources::ResourceManager;
use game::Game;
use gui::Gui;
use scripting::{Scripting, LuaChannel,};
use sfml::window::Style;
use sfml::graphics::RenderWindow;
use util::*;
use rlua::prelude::*;
use rlua::{Result,};


fn main() -> Result<()> {
    let (event_tx, event_rx, mut event_queue) = EventQueue::new();
    let mut window = SimpleWindow::new(
        RenderWindow::new(
            (800,600),
            "Ciso",
            Style::CLOSE,
            &Default::default(),
        ),
        event_tx.clone(),
    ); 
    let resource_manager = shared(ResourceManager::new());
    let scripting = shared(Scripting::new(LuaChannel::new(event_tx.clone())));
    let mut gui = Gui::new(
        event_rx.clone(),
        resource_manager.clone()
    );
    let mut game = Game::new(
        event_rx.clone(),
        resource_manager.clone()
    );

    println!("All systems initialized.");
    let mut elapsed = Instant::now();

    drop(event_tx);
    drop(event_rx);
    
    while window.is_open() {
        let dt = Instant::now().duration_since(elapsed).as_secs_f32();
        elapsed = Instant::now();
        // get input from window
        window.process_input();
        // Do all updates
        gui.update(dt);
        game.update(dt);
        // Render to the window
        window.begin();
        gui.draw(dt, &mut window);
        game.draw(dt, &mut window);
        window.end();

        event_queue.transmit();
    }

    Ok(())
    
}