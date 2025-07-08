#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

use alloc::boxed::Box;

use crate::scene_list::SCENES;

// By default no_std crates don't get alloc, so you won't be able to use things like Vec
// until you declare the extern crate. `agb` provides an allocator so it will all work
extern crate alloc;

mod game_state;
mod scene_list;
#[path = "traits/game_obj.rs"] mod game_obj;
#[path = "actors/actor_list.rs"] mod actor_list;

const DELTA : f32 = 1.0 / 59.73;

#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    let mut gfx = gba.graphics.get();
    let mut game_state = game_state::GameState::new();
    game_state.change_scene(scene_list::SCENES::TEST_SCENE);

    loop {
        let mut frame = gfx.frame();
        game_state.cycle_update(&mut frame);
        frame.commit();
    }
}