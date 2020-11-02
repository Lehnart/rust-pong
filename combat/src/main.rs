use crate::logic::Logic;
use engine::graphics::Window;
use crate::graphics::Graphics;
use std::time::SystemTime;
use crate::event::handle_event;
use crate::input::handle_input;

mod logic;
mod graphics;
mod event;
mod input;

pub const WINDOW_WIDTH : u32 = 600;
pub const WINDOW_HEIGHT : u32 = 700;

fn main() {
    let mut logic = Logic::new();

    let mut window = Window::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    let mut graphics = Graphics::new(WINDOW_WIDTH,WINDOW_HEIGHT);

    let mut previous = SystemTime::now();
    'game_loop: loop {
        let next = SystemTime::now();
        let dt = next.duration_since(previous).unwrap().as_secs_f32();
        previous = next;

        let event_pump = &mut window.event_pump;
        for event in event_pump.poll_iter() {
            handle_event(event, &mut logic);
        }
        handle_input(event_pump, &mut logic);


        logic.update(dt);
        if logic.is_over() {
            break 'game_loop;
        }

        graphics.update(&logic, &window);
        graphics.draw(&mut window);
    }
}
