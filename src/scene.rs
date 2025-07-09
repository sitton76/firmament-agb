use agb::fixnum::Vector2D;
use alloc::{boxed::Box, vec::Vec};
use crate::{actor, game_obj::GameObj};

#[derive(Clone, Copy)]
pub enum SCENES {
    TestScene,
    Map001,
    Map002
}

pub fn get_layout(scene: SCENES) -> Vec<Box<dyn GameObj>> {
    let mut new_obj_box: Vec<Box<dyn GameObj>> = Vec::new();
    match scene {
        SCENES::TestScene => {
            new_obj_box.push(Box::new(actor::player::Player::new(Vector2D { x: 0, y: 0 })));
            //new_obj_box.push(Box::new(actor::wall::Wall::new(Vector2D { x: 50, y: 50 }, Vector2D { x: 50, y: 200 })));
        },
        SCENES::Map001 => {
            new_obj_box.push(Box::new(actor::player::Player::new(Vector2D { x: 0, y: 0 })));
        },
        SCENES::Map002 => {
            new_obj_box.push(Box::new(actor::player::Player::new(Vector2D { x: 0, y: 0 })));
        },
    }
    return new_obj_box;
}