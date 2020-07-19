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
use events::{EventQueue,};
use rendering::{SimpleWindow, Renderer};
use resources::ResourceManager;
use game::Game;
use gui::Gui;
use scripting::Scripting;
use sfml::window::Style;
use sfml::graphics::RenderWindow;
use util::*;
use rlua::prelude::*;
use rlua::{Function, Lua, MetaMethod, Result, UserData, UserDataMethods, Variadic};


fn main() -> Result<()> {
    let mut event_queue = EventQueue::new();
    let mut window = SimpleWindow::new(
        RenderWindow::new(
            (800,600),
            "Ciso",
            Style::CLOSE,
            &Default::default(),
        )
    ); 
    let resource_manager = shared(ResourceManager::new());
    let scripting = shared(Scripting::new());
    let mut gui = Gui::new(&mut event_queue, scripting.clone(), resource_manager.clone());
    let mut game = Game::new(scripting.clone(), resource_manager.clone());
    game.test_script1();
    game.test_script2()?;

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

    Ok(())
    
}
