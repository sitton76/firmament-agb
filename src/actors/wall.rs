use agb::fixnum::{vec2, Rect};
use agb::input::Button;
use agb::{display::GraphicsFrame, fixnum::Vector2D};
use agb::include_aseprite;
use agb::display::object::Object;
use alloc::boxed::Box;
use crate::game_obj::{GameObj, ResponseType};
use crate::{global_data, DELTA};

include_aseprite!(
    mod sprites,
    "gfx/new_img.aseprite"
);

pub(crate) struct Wall {
    col: Rect<i32>,
    on_screen: bool,
    free_ready: bool
}

impl Wall {
    pub fn new(starting_pos: Vector2D<i32>, rect_size: Vector2D<i32>) -> Wall {
        Wall {
            col: Rect { position: starting_pos, size: rect_size },
            on_screen: true,
            free_ready: false
        }
    }

}

impl GameObj for Wall {
    fn on_screen(&self) -> bool {
        return self.on_screen;
    }

    fn check_to_free(&self) -> bool {
        return self.free_ready;
    }

    fn check_response_type(&self) -> ResponseType {
        return ResponseType::WALL;
    }

    fn get_collider(&self) -> Option<Rect<i32>> {
        return Some(self.col);
    }

    fn get_pos(&self) -> Option<Vector2D<i32>> {
        return Some(self.col.position);
    }
}