/*
    The scene manager, contains a enum list of all scenes in the game
    and a function to spawn them into the game_states obj_box
*/

use agb::fixnum::Vector2D;
use alloc::vec::Vec;
use crate::actor;

#[derive(Clone, Copy)]
pub enum SCENES {
    TestScene,
    Map001
}

pub enum BACKGROUNDS {
    BgImg1,
    BgImg2
}

pub fn get_layout(scene: SCENES) -> Vec<actor::Actor> {
    let mut new_obj_box: Vec<actor::Actor> = Vec::new();
    match scene {
        SCENES::TestScene => {
            // Test for the scene boundries and scrolling
            new_obj_box.push(actor::Actor::APlayer(Vector2D { x: 50, y: 50 }));
            new_obj_box.push(actor::Actor::AWall(Vector2D { x: 0, y: -80 }));
            new_obj_box.push(actor::Actor::AWall(Vector2D { x: 0, y: 160 }));
            new_obj_box.push(actor::Actor::AWall(Vector2D { x: -120, y: 50 }));
            new_obj_box.push(actor::Actor::AWall(Vector2D { x: 346, y: 50 }));
            new_obj_box.push(actor::Actor::AWall(Vector2D { x: 150, y: -20 }));
        },
        SCENES::Map001 => {
            new_obj_box.push(actor::Actor::APlayer(Vector2D { x: 50, y: 50 }));
            new_obj_box.push(actor::Actor::AWall(Vector2D { x: 100, y: 100 }));
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