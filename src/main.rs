#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]
extern crate alloc;

use agb::include_background_gfx;
use agb::display::tiled::VRAM_MANAGER;
include_background_gfx!(
    mod background,
    BG1 => deduplicate "gfx/background.aseprite",
);

use agb::display::{
    Priority,
    tiled::{RegularBackground, RegularBackgroundSize, TileFormat},
};

mod game_state;
mod scene;
mod actor;
mod game_obj;
mod global_data;

const DELTA : f32 = 1.0 / 59.73;

#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    let mut gfx = gba.graphics.get();
    VRAM_MANAGER.set_background_palettes(background::PALETTES);
    let mut game_state = game_state::GameState::new();
    game_state.change_scene(scene::SCENES::TestScene);


    let mut bg = RegularBackground::new(
        Priority::P3,
        RegularBackgroundSize::Background32x32,
        TileFormat::FourBpp
    );

    bg.fill_with(&background::BG1);

    loop {
        let mut frame = gfx.frame();
        game_state.cycle_update(&mut frame);
        
        bg.show(&mut frame);
        frame.commit();
    }
}