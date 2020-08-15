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
use rlua::{Table, Result,};

pub struct Panel<'s> {
    shape: RectangleShape<'s>,
    state: WidgetStateHandler,
    styles: StyleMap,
    children: Vec<Box<dyn Widget>>,
    id: u32
}

impl<'s> Panel<'s> {
    pub fn new(id: u32) -> Panel<'s> {
        let panel = Panel {
            shape: RectangleShape::new(),
            state: WidgetStateHandler::new(),
            styles: StyleMap::new(),
            children: vec![],
            id: id
        };
        panel
    }

    fn update_state(&mut self, new_state: WidgetState) {
        if let Some(style) = self.styles.get_style(&new_state) {
            self.shape.set_fill_color(style.background_color);
        }
    }

    pub fn set_properties(&mut self, properties: &Table) -> Result<()> {
        lua_get_pair(properties, "size").map(|v| self.shape.set_size(v)).ok();
        lua_get_pair(properties, "position").map(|v| self.shape.set_position(v)).ok();
        self.state.set_properties(&properties)?;
        Ok(())
    }

    // probably delete this later dude
    pub fn add_child(&mut self, panel: Box<Panel<'static>>) {
        self.children.push(panel);
    }

    pub fn from_table(t: Table) -> Result<Self> {
        let id: u32 = t.get("id")?;
        let properties: Table = t.get("properties")?;
        let mut panel = Panel::new(id);
        panel.set_properties(&properties)?;
        Ok(panel)
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

    fn handle_input(&mut self, handled: &mut bool, sf_event: &SFEvent) {
        for child in self.children.iter_mut() {
            child.handle_input(handled, sf_event);
        }
        if let Some(new_state) = self.state.handle_state(
            &self.shape.global_bounds(),
            handled,
            sf_event
        ) {
            self.update_state(new_state);
        }
    }

    fn widget_changed(&mut self, id: u32, table: &Table) {
        if id == self.id {
            if let Err(err) = self.set_properties(table) {
                println!("Could not set properties at id {}: {}", id, err);
            }
        } else {
            let children = &mut self.children;
            for child in children {
                child.widget_changed(id, table);
            }
        }
    }

    fn is_closed(&self) -> bool {
        self.state.closed
    }

    fn close(&mut self) {
        self.state.closed = true;
    }

}