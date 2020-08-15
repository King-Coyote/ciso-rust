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
use rlua::{Table, Result, Lua, Context, Value, Error as LuaError};
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
            Event::WidgetChanged(id) => {
                self.handle_event_widget_changed(id);
            },
            _ => {},
        }
    }

    fn handle_event_create(&mut self, id: u32) {
        let root_widgets = &mut self.root_widgets;
        match safe_context!(self.scripting, |ctx| -> Result<()> {
            let widget_table: Table = widget_table!(ctx, id);
            let widget = build_widget(widget_table)?;
            root_widgets.push(widget);
            Ok(())            
        }) {
            Err(err) => println!("Failed to create widget at gui level: \n{}", err),
            _ => {}
        }
    }

    fn handle_event_widget_changed(&mut self, id: u32) {
        let root_widgets = &mut self.root_widgets;
        if let Err(err) = safe_context!(self.scripting, |ctx| -> Result<()> {
            let new_props_table: Table = widget_table!(ctx, id);
            for widget in root_widgets {
                widget.widget_changed(id, &new_props_table);
            }
            Ok(())
        }) {
            println!("Couldn't get widget table for change event: {}", err);
        };
        println!("Widget with id {} changed", id);
    }
}

fn lua_preamble(scripting: &Shared<Scripting>) -> Result<()> {
    safe_context!(scripting, |ctx| {
        let gui_table = ctx.create_table()?;
        gui_table.set("num_widgets", 1)?;
        gui_table.set("widgets", ctx.create_table()?)?;

        create_widget_metatable(&ctx)?;

        let lua_add_widget = ctx.create_function(|ctx, (this, t): (Table, Table)| {
            let num: u32 = this.raw_get("num_widgets")?;
            t.raw_set("id", num)?;
            // give the table the widget metatable
            t.set_metatable(ctx.globals().get("Widget_MT")?);
            // add the actual widget info to the widgets table
            this.get::<&str, Table>("widgets")?
                .set(num, t.clone())?;
            // send an event telling rust that the widget has been made
            ctx.globals().get::<&str, LuaChannel>("EventChannel")?
                .send(Event::CreateGui(num))?;
            this.raw_set("num_widgets", num + 1)?;
            
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
    widget_mt.set("set_properties", ctx.create_function(|ctx, (this, new_props): (Table, Table)| {
        // TODO can generalise this into a has-invalid-args type situation
        if new_props.contains_key("id")? {
            // TODO cover with tests
            return wrap_error_for_lua(Error::InvalidArgs("Cannot include 'id' in set_properties.".to_owned()))
        }
        let id: u32 = this.get("id")?;
        lua_spread_tables(&this, new_props)?;
        ctx.globals().get::<_, LuaChannel>("EventChannel")?
            .send(Event::WidgetChanged(id))?;
        Ok(())
    })?)?;
    widget_mt.set("__index", widget_mt.clone())?;
    ctx.globals().set("Widget_MT", widget_mt)?;
    Ok(())
}
