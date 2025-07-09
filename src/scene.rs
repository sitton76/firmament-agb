/*
    The scene manager, contains a enum list of all scenes in the game
    and a function to spawn them into the game_states obj_box
*/

use agb::fixnum::Vector2D;
use alloc::{boxed::Box, vec::Vec};
use crate::{actor, game_obj::GameObj};

#[derive(Clone, Copy)]
pub enum SCENES {
    TestScene,
    Map001
}

#[derive(Clone, Copy)]
pub enum BACKGROUNDS {
    BgImg1,
    BgImg2
}

pub fn get_layout(scene: SCENES) -> Vec<Box<dyn GameObj>> {
    let mut new_obj_box: Vec<Box<dyn GameObj>> = Vec::new();
    match scene {
        SCENES::TestScene => {
            new_obj_box.push(Box::new(actor::player::Player::new(
                Vector2D { x: 50, y: 50 }))
            );
            new_obj_box.push(Box::new(actor::wall::Wall::new(
                Vector2D { x: 100, y: 100 }))
            );
        },
        SCENES::Map001 => {
            new_obj_box.push(Box::new(actor::player::Player::new(
                Vector2D { x: 50, y: 50 }))
            );
            new_obj_box.push(Box::new(actor::wall::Wall::new(
                Vector2D { x: 100, y: 100 }))
            );
        }
    }
    return new_obj_box;
}

pub fn get_bg_val(scene: SCENES) -> Option<BACKGROUNDS> {
    match scene {
        SCENES::TestScene => {
            return Some(BACKGROUNDS::BgImg1);
        },
        SCENES::Map001 => {
            return Some(BACKGROUNDS::BgImg2);
        }
    };

}