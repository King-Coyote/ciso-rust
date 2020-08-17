mod events;
mod rendering;
mod resources;
mod gui;
mod game;
mod scripting;
mod util;
mod macros;
mod error;

extern crate sfml;
extern crate rlua;

use std::{
    time::{Instant,}
};
use events::{EventQueue,};
use rendering::{Renderer};
use resources::ResourceManager;
use game::Game;
use gui::Gui;
use scripting::*;
use sfml::window::Style;
use sfml::graphics::RenderWindow;
use util::*;
use rlua::{Result,};


fn main() -> Result<()> {
    let (event_tx, event_rx, event_queue) = EventQueue::new();
    let mut window = Renderer::new(
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
        scripting.clone(),
        event_rx.clone(),
        resource_manager.clone()
    );
    let mut game = Game::new(
        scripting.clone(),
        event_rx.clone(),
        resource_manager.clone()
    );

    println!("All systems initialized.");
    let mut elapsed = Instant::now();

    drop(event_tx);
    drop(event_rx);

    safe_context!(scripting, |ctx| -> Result<()> {
        exec_lua_file(&ctx, "scripts/child_widget_test.lua")?;
        Ok(())
    }).expect("Could not run lua file.");
    
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