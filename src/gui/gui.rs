use crate::events::*;
use crate::gui::Widget;
use crate::gui::Panel;
use crate::rendering::*;
use crate::util::*;
use crate::resources::ResourceManager;
use crate::scripting::{Scripting, LuaChannel,};
use std::sync::{Arc, Mutex};
use rlua::{Table, Result, Lua, Scope};
use crossbeam_channel::Receiver;
use sfml::window::Event as SFEvent;

pub struct Gui {
    scripting: Shared<Scripting>,
    event_rx: Receiver<Event>,
    root_widgets: Vec<Box<dyn Widget>>,
    resource_manager: Shared<ResourceManager>,
}

impl Gui {
    pub fn new(
        scripting: Shared<Scripting>,
        event_rx: Receiver<Event>,
        resource_manager: Shared<ResourceManager>
    ) -> Self {
        lua_preamble(&scripting.lock().unwrap().lua).expect("Failed to add gui preamble to lua.");
        let mut gui = Gui {
            scripting: scripting,
            event_rx: event_rx,
            root_widgets: vec!(),
            resource_manager: resource_manager
        };
        return gui;
    }

    pub fn update(&mut self, dt: f32) {
        loop {
            match self.event_rx.try_recv() {
                Ok(event) => {
                    self.handle_event(event);
                },
                _ => break
            }
        }
    }

    pub fn draw(&mut self, dt: f32, renderer: &mut Renderer) {
        self.root_widgets.retain(|w| {
            w.update(dt);
            w.draw(dt, renderer);
            !w.is_closed()
        });
    }

    // fn add_widget(&mut self, widget_table: &Table, scope: &Scope) {
    //     let mut wid = Box::new(Panel::new((100.0, 100.0), (0.0,0.0), "Parent"));
    //     wid.add_child(Box::new(Panel::new((50.0, 50.0), (0.0, 0.0), "Child")));
    //     self.root_widgets.push(wid);
    // }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::CreateGui(id) => self.handle_event_create(id),
            Event::Input(sf_event) => {
                let mut handled = false;
                for widget in self.root_widgets.iter_mut() {
                    if handled {
                        break;
                    }
                    widget.handle_input(&mut handled, &sf_event);
                }
            }
        }
    }

    fn handle_event_create(&mut self, id: u32) {
        let mut root_widgets = &mut self.root_widgets;
        self.scripting.lock().unwrap().lua.context(|ctx| -> Result<()> {
            ctx.scope(|scope| {
                let globals = ctx.globals();
                let widget_table: Table = globals
                    .get::<&str, Table>("GUI")?
                    .get::<&str, Table>("widgets")?
                    .get::<u32, Table>(id)?;
                root_widgets.push(Box::new(Panel::new((100.0, 100.0), (40.0, 40.0), id)));
                println!("Created gui with id {}", id);
                Ok(())
            })?;
            Ok(())
            
        }).expect("Failed to create widget");
    }
}

fn lua_preamble(lua: &Lua) -> Result<()> {
    lua.context(|ctx| {
        ctx.load(r#"
            Gui = {
                add_widget = function()
                    print("test succeeded")
                end
            }
        "#).exec()?;
        Ok(())
    })?;
    Ok(())
}