use crate::{
    gui::*,
    rendering::*,
    util,
};
use rlua::{
    Context,
    Result,
    Table,
    Value,
    Error,
};
use sfml::{
    window::Event as SFEvent,
};
use std::{
    sync::atomic::{AtomicU32, Ordering,},
};

static mut ID: AtomicU32 = AtomicU32::new(0);

// the widget trait obj is responsible for visual things eg drawing, positioning, shapes
// this is responsible for delegating behaviour and holding child nodes
// the state handler is for working more directly with the lua table describing the widget, and controlling behaviour eg clicking
pub struct WidgetNode {
    id: u32,
    widget: Box<dyn Widget>,
    state_handler: WidgetStateHandler,
    children: Vec<WidgetNode>,
    styles: StyleMap,
}

impl WidgetNode {

    pub fn new<'lua>(ctx: &Context<'lua>, widget_table: Table<'lua>) -> Result<Self> {
        let properties: Table = widget_table.get("properties")?;
        let style_table: Table = properties.get("style")?;

        let mut id: u32;
        unsafe {id = ID.fetch_add(1, Ordering::Relaxed);}
        widget_table.set("id", id)?;

        let mut node = WidgetNode {
            id: id,
            widget: widget::build_widget(ctx, &widget_table)?,
            state_handler: WidgetStateHandler::new(ctx, &widget_table)?,
            children: vec![],
            styles: StyleMap::new(&style_table),
        };
        if let Ok(children_table) = widget_table.get::<_, Table>("children") {
            for pair in children_table.pairs::<Value, Table>() {
                let (_, child) = pair?;
                child.set("parent", widget_table.clone())?;
                node.children.push(WidgetNode::new(ctx, child)?);
            }
        }
        node.set_properties(ctx, &properties)?;
        let style = node.styles.get_style(&node.state_handler.state);
        node.widget.update_style(&node.styles.get_style(&node.state_handler.state));
        Ok(node)
    }

    pub fn draw(&self, dt: f32, renderer: &mut Renderer) {
        self.widget.draw(dt, renderer);
        for child in self.children.iter() {
            child.draw(dt, renderer);
        }
    }

    pub fn update(&self, dt: f32) {
        self.widget.update(dt);
    }

    pub fn handle_input(&mut self, ctx: &Context, handled: &mut bool, sf_event: &SFEvent) {
        for child in self.children.iter_mut() {
            child.handle_input(ctx, handled, sf_event);
        }
        if let Some(new_state) = self.state_handler.handle_state(
            &self.widget.get_bounds(),
            handled,
            sf_event,
            ctx,
        ) {
            self.widget.update_style(self.styles.get_style(&new_state));
        }
    }

    pub fn properties_changed<'lua>(&mut self, ctx: &Context<'lua>, id: u32, new_props: &Table<'lua>) -> Result<()> {
        if id == self.id {
            self.set_properties(ctx, new_props)?;
        } else {
            for child in self.children.iter_mut() {
                child.properties_changed(ctx, id, new_props)?;
            }
        }
        Ok(())
    }

    pub fn is_closed(&self) -> bool {
        self.state_handler.closed
    }

    // position is done here because children need to be updated relative to parent - hence the 
    pub fn set_properties<'lua>(&mut self, ctx: &Context<'lua>, new_props: &Table<'lua>) -> Result<()> {
        util::lua_get_pair(new_props, "position").map(|v: (f32, f32)| {
            let current_pos = self.widget.get_position();
            let delta: (f32, f32) = (v.0 - current_pos.x, v.1 - current_pos.y);
            self.translate(delta);
        }).ok();
        self.widget.set_properties(ctx, new_props);
        self.state_handler.set_properties(ctx, new_props.clone())?;
        Ok(())
    }

    pub fn translate(&mut self, delta: (f32, f32)) {
        self.widget.translate(delta);
        for child in self.children.iter_mut() {
            child.translate(delta);
        }
    }

}