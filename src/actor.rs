/*
    Contains a list of actors, each time we make a new one add it here.
*/

use agb::fixnum::Vector2D;
use alloc::boxed::Box;
use crate::game_obj::GameObj;

#[path = "actors/player.rs"] pub(crate) mod player;
#[path = "actors/wall.rs"] pub(crate) mod wall;
//#[path = "actors/tilesheet.rs"] pub(crate) mod tilesheet;

#[derive(Clone, Copy)]
pub(crate) enum Actor {
    APlayer(Vector2D<i32>),
    AWall(Vector2D<i32>)
}

pub fn spawn_actor(actor_type: Actor) -> Box<dyn GameObj> {
    match actor_type {
        Actor::APlayer(pos) => return Box::new(player::Player::new(pos)),
        Actor::AWall(pos) => return Box::new(wall::Wall::new(pos)),
    }
}