use crate::game::{Position, Movement, Appearance};
use crate::rendering::{Renderer};

#[derive(Default,)]
pub struct MovementSystem {}

impl MovementSystem {
    pub fn new() -> Self {
        MovementSystem{

        }
    }

    pub fn update(&mut self, dt: f32, movements: &[Movement], positions: &mut [Position]) {

    }
}

#[derive(Default,)]
pub struct AppearanceSystem {}

impl AppearanceSystem {
    pub fn new() -> Self {
        AppearanceSystem{

        }
    }

    pub fn update(&mut self, dt: f32, appearances: &[Appearance]) {

    }

    pub fn draw(&self, dt: f32, appearances: &[Appearance], renderer: &mut impl Renderer) {
        
    }
}