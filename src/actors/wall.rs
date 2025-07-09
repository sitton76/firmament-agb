use agb::fixnum::Rect;
use agb::fixnum::Vector2D;
use agb::include_aseprite;
use crate::game_obj::{GameObj, ResponseType};

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