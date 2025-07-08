use alloc::{boxed::Box, vec::Vec};
use crate::{actor_list, game_obj::GameObj};


#[derive(Clone, Copy)]
pub enum SCENES {
    TEST_SCENE,
    MAP_001,
    MAP_002
}

pub fn get_layout(scene: SCENES) -> Vec<Box<dyn GameObj>> {
    let mut new_obj_box: Vec<Box<dyn GameObj>> = Vec::new();
    match scene {
        SCENES::TEST_SCENE => {
            new_obj_box.push(Box::new(actor_list::player::Player::new()));
        },
        SCENES::MAP_001 => {
            new_obj_box.push(Box::new(actor_list::player::Player::new()));
        },
        SCENES::MAP_002 => {
            new_obj_box.push(Box::new(actor_list::player::Player::new())); 
        },
    }
    return new_obj_box;
}