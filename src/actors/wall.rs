use agb::display::object::Object;
use agb::display::GraphicsFrame;
use agb::fixnum::Rect;
use agb::fixnum::Vector2D;
use agb::include_aseprite;
use crate::game_obj::{GameObj, ResponseType};

include_aseprite!(
    mod sprites,
    "gfx/new_img.aseprite"
);

pub(crate) struct Wall {
    object: Object,
    col: Rect<i32>,
    on_screen: bool,
    free_ready: bool
}

impl Wall {
    pub fn new(starting_pos: Vector2D<i32>) -> Wall {
        Wall {
            object: Object::new(sprites::WALL.sprite(0)),
            col: Rect { position: starting_pos, size: Vector2D { x: 16, y: 16 } },
            on_screen: true,
            free_ready: false
        }
    }

}

impl GameObj for Wall {
    fn update(&mut self, _globals: &mut crate::global_data::GlobalData) {
        self.object.set_pos(self.col.position);
    }

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

    fn draw(&self, frame: &mut GraphicsFrame) {
        self.object.show(frame);
    }
}