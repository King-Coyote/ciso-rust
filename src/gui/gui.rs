use crate::{
    events::*,
    gui::{Widget, build_widget,},
    rendering::*,
    util::*,
    resources::ResourceManager,
    scripting::{Scripting, LuaChannel,},
    error::Error,
};
#[macro_use]
use crate::{safe_context, widget_table};
use rlua::{Table, Result, Lua, Context, Value, Error as LuaError, RegistryKey,};
use crossbeam_channel::Receiver;

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
        lua_preamble(&scripting).expect("Failed to add gui preamble to lua.");
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
        let mut closed_widgets = false;
        self.root_widgets.retain(|w| {
            w.update(dt);
            w.draw(dt, renderer);
            if w.is_closed() {
                closed_widgets = true;
                return false;
            }
            true
        });
        if closed_widgets {
            let lua = &self.scripting.lock().unwrap().lua;
            lua.context(|ctx| {
                ctx.expire_registry_values();
                lua.gc_collect().unwrap();
            });
        }
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::CreateGui(key) => self.handle_event_create(key),
            Event::Input(sf_event) => {
                let mut handled = false;
                for widget in self.root_widgets.iter_mut() {
                    if handled {
                        break;
                    }
                    safe_context!(self.scripting, |ctx| {
                        widget.handle_input(&ctx, &mut handled, &sf_event);
                    })
                }
            }
            Event::WidgetChanged(id, key) => {
                if let Err(err) = self.handle_event_widget_changed(id, key) {
                    println!("Failed to change widget {}: {}", id, err);
                }
            },
            _ => {},
        }
    }

    fn handle_event_create(&mut self, key: RegistryKey) {
        let root_widgets = &mut self.root_widgets;
        match safe_context!(self.scripting, |ctx| -> Result<()> {
            let widget = build_widget(&ctx, key)?;
            root_widgets.push(widget);
            Ok(())            
        }) {
            Err(err) => println!("Failed to create widget at gui level: \n{}", err),
            _ => {}
        }
    }

    fn handle_event_widget_changed(&mut self, id: u32, key: RegistryKey) -> Result<()> {
        let root_widgets = &mut self.root_widgets;
        safe_context!(self.scripting, |ctx| {
            let new_props_table: Table = ctx.registry_value(&key)?;
            for widget in root_widgets {
                widget.widget_changed(&ctx, id, &new_props_table)?;
            }
            Ok(())
        })?;
        Ok(())
    }
}

fn lua_preamble(scripting: &Shared<Scripting>) -> Result<()> {
    safe_context!(scripting, |ctx| {
        let gui_table = ctx.create_table()?;
        gui_table.set("num_widgets", 1)?;
        create_widget_metatable(&ctx)?;

        let lua_add_widget = ctx.create_function(|ctx, (this, t): (Table, Table)| {
            let num_widgets: u32 = this.get("num_widgets")?;
            t.set("id", num_widgets + 1);
            // give the table the widget metatable
            t.set_metatable(ctx.globals().get("Widget_MT")?);
            let key = ctx.create_registry_value(t.clone())?;
            // send an event telling rust that the widget has been made
            ctx.globals().get::<&str, LuaChannel>("EventChannel")?
                .send(Event::CreateGui(key))?;
            this.set("num_widgets", num_widgets + 1)?;
            Ok(t)
        })?;
        gui_table.set("add_widget", lua_add_widget)?;

        ctx.globals().set("Gui", gui_table)?;
        Ok(())
    })?;
    Ok(())
}

fn create_widget_metatable(ctx: &Context) -> Result<()> {
    let widget_mt = ctx.create_table()?;

    // set up functions callable on widget tables
    widget_mt.set("set_properties", ctx.create_function(|ctx, (this, new_props): (Table, Table)| {
        let id: u32 = this.get("id")?;
        let props_key = ctx.create_registry_value(new_props)?;
        ctx.globals().get::<_, LuaChannel>("EventChannel")?
            .send(Event::WidgetChanged(id, props_key))?;
        Ok(())
    })?)?;

    widget_mt.set("__index", widget_mt.clone())?;
    ctx.globals().set("Widget_MT", widget_mt)?;
    Ok(())
}
