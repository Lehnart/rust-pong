use crate::logic::Logic;
use sdl2::keyboard::Scancode;

pub fn handle_input(event_pump: &sdl2::EventPump, logic: &mut Logic){
    if event_pump.keyboard_state().is_scancode_pressed(Scancode::Left){
        logic.left_tank.turn_left();
    }
    if event_pump.keyboard_state().is_scancode_pressed(Scancode::Right){
        logic.left_tank.turn_right();
    }
}