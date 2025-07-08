#![allow(dead_code, unused_variables)]
use agb::{display::GraphicsFrame, fixnum::Vector2D};
use alloc::boxed::Box;

pub(crate) enum ResponseType {
    NONE,
    DAMAGE,
    WALL,
    PLAYER
}

pub trait GameObj {
    fn ready(&mut self) { //Called after entering the object_box
        return;
    }

    fn update(&mut self) { //Updates the object
        return;
    }

    fn on_screen(&self) -> bool { //Check if a object is on screen or not.
        return false;
    }

    fn check_to_free(&self) -> bool { //Checks to see if the object should be freed
        return false;
    }

    fn check_collision(&mut self, other: &Box<dyn GameObj>, delta: f32) -> ResponseType { //Handles the local collision detection.
        return ResponseType::NONE;
    }

    fn check_response_type(&self) -> ResponseType { //Returns the response type the object uses.
        return ResponseType::NONE;
    }

    fn handle_response(&mut self, response: ResponseType) { //Handles the reaction from the object being checked in "check_collision()"
        return;
    }

    fn get_pos(&self) -> Option<Vector2D<i32>> {  //Gets the position of the object.
        return None;
    }

    fn draw(&self, frame: &mut GraphicsFrame) { //Draws the object
        return;
    }
}