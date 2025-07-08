#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]
extern crate alloc;

mod game_state;
mod scene;
mod actor;
mod game_obj;
mod global_data;

const DELTA : f32 = 1.0 / 59.73;

#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    let mut gfx = gba.graphics.get();
    let mut game_state = game_state::GameState::new();
    game_state.change_scene(scene::SCENES::TestScene);
    loop {
        let mut frame = gfx.frame();
        game_state.cycle_update(&mut frame);
        frame.commit();
    }
}