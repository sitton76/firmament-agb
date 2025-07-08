use alloc::{boxed::Box, vec::Vec};
use crate::{actor_list, game_obj::GameObj};

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
            new_obj_box.push(Box::new(actor_list::player::Player::new()));
        },
        SCENES::Map001 => {
            new_obj_box.push(Box::new(actor_list::player::Player::new()));
        },
        SCENES::Map002 => {
            new_obj_box.push(Box::new(actor_list::player::Player::new()));
        },
    }
    return new_obj_box;
}