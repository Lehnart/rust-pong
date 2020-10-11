mod graphics;
mod event;
mod collide;
mod audio;
mod logic;

use std::time::SystemTime;

use graphics::Graphics;
use graphics::Window;
use logic::Logic;
use audio::Audio;
use event::handle_event;
use collide::Collide;


fn main() {

    let mut logic = Logic::new();
    let mut window = Window::new(600, 600);
    let mut graphics = Graphics::new(&window.canvas);

    let audio = Audio::new();
    let mut collide = Collide::new();

    let mut previous = SystemTime::now();
    'game_loop: loop {

        let next = SystemTime::now();
        let dt = next.duration_since(previous).unwrap().as_secs_f32();
        previous = next;

        let event_pump = &mut window.event_pump;
        for event in event_pump.poll_iter() {
            handle_event(event, &mut logic);
        }

        logic.update(dt);
        if logic.is_over(){
            break 'game_loop
        }

        collide.collide(&mut logic, dt);

        audio.update(&logic, &collide);

        graphics.update(&logic);
        graphics.draw(&mut window.canvas);


    }
}
