use crate::events::*;
use crate::gui::Widget;
use crate::gui::Panel;
use crate::rendering::*;
use crate::util::*;
use crate::resources::ResourceManager;
use crate::scripting::{Scripting, LuaChannel,};
use std::sync::{Arc, Mutex};
use rlua::{Table, Result, Lua, Scope, UserData, Value};
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
        match self.scripting.lock().unwrap().lua.context(|ctx| -> Result<()> {
            ctx.scope(|scope| {
                let globals = ctx.globals();
                let widget_table: Table = globals
                    .get::<&str, Table>("Gui")?
                    .get::<&str, Table>("widgets")?
                    .get::<u32, Table>(id)?;
                // replace with builder later on
                let size: (f32, f32) = table_to_pair(widget_table.get("size")?)?;
                let position: (f32, f32) = table_to_pair(widget_table.get("position")?)?;
                root_widgets.push(Box::new(Panel::new(
                    size, 
                    position, 
                    id
                )));
                Ok(())
            })?;
            Ok(())
            
        }) {
            Err(err) => println!("Failed to create widget: \n{}", err),
            _ => {}
        }
    }
}

fn lua_preamble(lua: &Lua) -> Result<()> {
    lua.context(|ctx| {
        let gui_table = ctx.create_table()?;
        gui_table.set("num_widgets", 1)?;
        gui_table.set("widgets", ctx.create_table()?)?;

        let lua_add_widget = ctx.create_function(|ctx, (this, t): (Table, Table)| {
            let num: u32 = this.raw_get("num_widgets")?;
            t.raw_set("id", num)?;
            // add the actual widget info to the widgets table
            this.get::<&str, Table>("widgets")?
                .set(num, t)?;
            let event_channel: LuaChannel = ctx.globals().get("EventChannel")?;
            ctx.globals().get::<&str, LuaChannel>("EventChannel")?
                .send(Event::CreateGui(num))?;
            this.raw_set("num_widgets", num + 1)?;
            Ok(())
        })?;
        gui_table.set("add_widget", lua_add_widget)?;

        ctx.globals().set("Gui", gui_table)?;
        Ok(())
    })?;
    Ok(())
}