use crate::{
    gui::{
        Widget, 
        WidgetStateHandler,
        WidgetState,
        StyleMap,
    },
    rendering::*,
    util::*,
};
use sfml::graphics::{
    RectangleShape, 
    Transformable, 
    Color,
    Shape,
};
use crate::rendering::*;
use sfml::window::Event as SFEvent;
use sfml::system::{Vector2f,};
use rlua::{Table, Result, Context, RegistryKey};

pub struct Panel<'s> {
    shape: RectangleShape<'s>,
    state: WidgetStateHandler,
    styles: StyleMap,
    children: Vec<Box<dyn Widget>>,
    id: u32
}

impl<'s> Panel<'s> {

    pub fn new<'lua>(ctx: &Context<'lua>, widget_table: Table<'lua>) -> Result<Self> {
        let properties: Table = widget_table.get("properties")?;
        let id: u32 = widget_table.get("id")?;
        let mut panel = Panel {
            shape: RectangleShape::new(),
            state: WidgetStateHandler::new(ctx, widget_table)?,
            styles: StyleMap::new(),
            children: vec![],
            id: id
        };
        panel.set_properties(ctx, &properties)?;
        Ok(panel)
    }

    fn update_state(&mut self, new_state: WidgetState) {
        if let Some(style) = self.styles.get_style(&new_state) {
            self.shape.set_fill_color(style.background_color);
        }
    }

    pub fn set_properties<'lua>(&mut self, ctx: &Context<'lua>, new_props: &Table<'lua>) -> Result<()> {
        lua_get_pair(new_props, "size").map(|v| self.shape.set_size(v)).ok();
        lua_get_pair(new_props, "position").map(|v| self.shape.set_position(v)).ok();
        self.state.set_properties(ctx, new_props.clone())?;
        Ok(())
    }

    // probably delete this later dude
    pub fn add_child(&mut self, panel: Box<Panel<'static>>) {
        self.children.push(panel);
    }


}

impl<'s> Widget for Panel<'s>
{
    fn draw(&self, dt: f32, renderer: &mut Renderer) {
        renderer.draw_shape(&self.shape);
        for child in self.children.iter() {
            child.draw(dt, renderer);
        }
    }

    fn update(&self, dt: f32) {

    }

    fn handle_input(&mut self, ctx: &Context, handled: &mut bool, sf_event: &SFEvent) {
        for child in self.children.iter_mut() {
            child.handle_input(ctx, handled, sf_event);
        }
        if let Some(new_state) = self.state.handle_state(
            &self.shape.global_bounds(),
            handled,
            sf_event,
            ctx,
        ) {
            self.update_state(new_state);
        }
    }

    fn widget_changed<'lua>(&mut self, ctx: &Context<'lua>, id: u32, new_props: &Table<'lua>) -> Result<()> {
        if id == self.id {
            self.set_properties(ctx, new_props)?;
        } else {
            let children = &mut self.children;
            for child in children {
                child.widget_changed(ctx, id, new_props)?;
            }
        }
        Ok(())
    }

    fn is_closed(&self) -> bool {
        self.state.closed
    }

    fn close(&mut self) {
        self.state.closed = true;
    }

}