use sdl2::surface::Surface;

use engine::geometry::{AsRect, Rect};
use engine::physics::{Position, Solid, Velocity};

pub const SPACESHIP_WIDTH : f32 = 0.065;
pub const SPACESHIP_HEIGHT : f32 = 0.0256;
pub const SPACESHIP_STARTING_POSITION_X0 : f32 = 0.45;
pub const SPACESHIP_STARTING_POSITION_Y0 : f32 = 0.9;


pub const SPACESHIP_SPEED : f32 = 0.75;
pub const SPACESHIP_EXPLOSION_DURATION : f32 = 1.;

pub struct Spaceship{
    solid : Solid,
    is_destroyed  : bool,
    delay_since_explosion : f32,
}

impl Spaceship{
    pub fn new() -> Spaceship {
        let position = Position::new(SPACESHIP_STARTING_POSITION_X0, SPACESHIP_STARTING_POSITION_Y0);
        let velocity = Velocity::new(0.,0.);
        let limit = Rect::new(0.,0.,1.,1.);

        Spaceship {
            solid : Solid::new(position, velocity, SPACESHIP_WIDTH,SPACESHIP_HEIGHT, limit),
            is_destroyed  : false,
            delay_since_explosion : 0.,
        }
    }

    pub fn accelerate(&mut self) {
        let pv = self.solid.vel.copy();
        self.solid.vel.set_vx(pv.vx() + SPACESHIP_SPEED);
    }

    pub fn decelerate(&mut self) {
        let pv = self.solid.vel.copy();
        self.solid.vel.set_vx(pv.vx() - SPACESHIP_SPEED);
    }

    pub fn update(&mut self, dt : f32){
        self.solid.update(dt);
    }

}

impl AsRect for Spaceship {
    fn as_rect(&self) -> Rect {
        self.solid.as_rect()
    }
}

/// Logic is a structure that contains all entities from the game.
pub struct Logic {
    pub spaceship : Spaceship,
    is_over: bool,
}

impl Logic {
    /// Create a new game logic with default values for game settings
    pub fn new() -> Logic {
        Logic {
            spaceship : Spaceship::new(),
            is_over: false,
        }
    }

    /// Update each entity of a delta of time and check if the game is over.
    pub fn update(&mut self, dt: f32) {
        self.spaceship.update(dt);
    }

    /// Set the game over
    pub fn over(&mut self) {
        self.is_over = true;
    }

    /// Is the game over?
    pub fn is_over(&self) -> bool {
        self.is_over
    }
}