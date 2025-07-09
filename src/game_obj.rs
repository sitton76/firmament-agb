#![allow(dead_code, unused_variables)]

/*
    The game object trait, all "actors" should implement this trait
    but does not need to implement all functions if the default
    implementaiton works.
*/

use agb::{display::GraphicsFrame, fixnum::{Rect, Vector2D}};
use alloc::boxed::Box;
use crate::global_data;

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

    fn update(&mut self, globals: &mut global_data::GlobalData) { //Updates the object
        return;
    }

    fn simple_update(&mut self, globals: &mut global_data::GlobalData) { // Updates the object when offscreen, should use more simple logic if needed, otherwise can forward the call to update()
        // Can also use this to trigger off screen specific logic, such as triggering a timer to reset a enemy respawn and updating it.
        self.update(globals);
    }

    fn on_screen(&self) -> bool { //Check if a object is on screen or not.
        // Renders each object off screen until its 16 pixels off screen.
        match self.get_pos() {
            Some(pos) => {
                let inside_x_range = (pos.x > -16) && (pos.x < 256);
                let inside_y_range = (pos.y > -16) && (pos.y < 176);
                return inside_x_range && inside_y_range;
            },
            None => return true,
        }
    }

    fn check_to_free(&self) -> bool { //Checks to see if the object should be freed
        return false;
    }

    fn check_collision(&mut self, other: &Box<dyn GameObj>) -> ResponseType { //Handles the local collision detection.
        return ResponseType::NONE;
    }

    fn check_response_type(&self) -> ResponseType { //Returns the response type the object uses.
        return ResponseType::NONE;
    }

    fn handle_response(&mut self, response: ResponseType) { //Handles the reaction from the object being checked in "check_collision()"
        return;
    }

    fn get_collider(&self) -> Option<Rect<i32>> { //Gets the collider for the object, or None if there is no collider
        return None;
    }

    fn get_pos(&self) -> Option<Vector2D<i32>> {  //Gets the position of the object.
        return None;
    }

    fn draw(&self, frame: &mut GraphicsFrame) { //Draws the object
        return;
    }
}