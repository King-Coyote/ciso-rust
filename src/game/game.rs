use crate::events::*;
use crate::rendering::*;
use crate::game::*;
use crate::util::*;
use crate::resources::ResourceManager;
use crate::scripting::Scripting;
use rlua::prelude::*;
use rlua::Result;

const NUM_ENT: u32 = 10;
/// Represents the game as an ECS system
pub struct Game<'a> {
    scripting: Shared<Scripting>,
    resource_manager: Shared<ResourceManager>,

    positions: Vec<Position>,
    appearances: Vec<Appearance<'a>>,
    movements: Vec<Movement>,

    sys_movement: MovementSystem,
    sys_appearance: AppearanceSystem,
}

impl<'a> Game<'a> {
    pub fn new(
        scripting: Shared<Scripting>, 
        resource_manager: Shared<ResourceManager>
    ) -> Self {
        Game{
            scripting: scripting,
            resource_manager: resource_manager,

            positions: vec![],
            appearances: vec![],
            movements: vec![],

            sys_movement: Default::default(),
            sys_appearance: Default::default(),
        }
    }

    pub fn update(&mut self, dt: f32, event_queue: &mut EventQueue) {
        self.sys_movement.update(dt, &self.movements, &mut self.positions);
        self.sys_appearance.update(dt, &self.appearances);
    }

    pub fn draw(&self, dt: f32, renderer: &mut impl Renderer) {
        self.sys_appearance.draw(dt, &self.appearances, renderer);
    }

    pub fn test_script1(&self) {
        let lua = &self.scripting.lock().unwrap().lua;
        lua.context(|ctx| {
            ctx.load(r#"
                print("fuck you")
            "#).eval::<()>();
        });
    }

    pub fn test_script2(&self) -> Result<()> {
        let lua = &self.scripting.lock().unwrap().lua;
        lua.context(|ctx| {
            let durr = ctx.create_function(|_, (x, y): (i32, i32)| {
                Ok(x + y)
            })?;
            let globals = ctx.globals();
            globals.set("durr", durr)?;

            let n = ctx.load(r#"durr(1, 2)"#).eval::<i32>()?;
            println!("Result of lua func: {}", n);

            Ok(())
        })?;
        Ok(())
    }

    pub fn test_script3(&self) -> Result<()> {

        Ok(())
    }
}