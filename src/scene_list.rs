use alloc::{boxed::Box, vec::Vec};
use crate::{actor_list, game_obj::GameObj};


#[derive(Clone, Copy)]
pub enum SCENES {
    NONE,
    TEST_SCENE,
    MAP_001,
    MAP_002
}

pub fn get_layout(scene: SCENES) -> Option<Vec<Box<dyn GameObj>>> {
    let mut new_obj_box: Vec<Box<dyn GameObj>> = Vec::new();

    match scene {
        SCENES::NONE => {
            return None;
        },
        SCENES::TEST_SCENE => {
            new_obj_box.push(Box::new(actor_list::player::Player::new()));
            return Some(new_obj_box);
        },
        SCENES::MAP_001 => {
            return Some(new_obj_box);
        },
        SCENES::MAP_002 => {
            return Some(new_obj_box);
        },
    }
}