use agb::input::{ButtonController, Button};
use agb::{display::GraphicsFrame, fixnum::Vector2D};
use agb::include_aseprite;
use agb::display::object::Object;
use crate::game_obj::{GameObj, ResponseType};
use crate::DELTA;

include_aseprite!(
    mod sprites,
    "gfx/new_img.aseprite"
);

pub(crate) struct Player {
    object: Object,
    pos: Vector2D<i32>,
    prev_pos: Vector2D<i32>,
    speed: f32,
    on_screen: bool,
    free_ready: bool
}

impl Player {
    pub fn new() -> Player {
        Player {
            object: Object::new(sprites::TEST_PLAYER.sprite(0)),
            pos: Vector2D {x: 0, y: 0},
            prev_pos: Vector2D { x: 0, y: 0 },
            speed: 100.0,
            on_screen: true,
            free_ready: false
        }
    }

    fn handle_input(&mut self, controller: &ButtonController) {
        if controller.is_pressed(Button::UP) {
            self.pos.y -= (self.speed * DELTA) as i32;
        } else if controller.is_pressed(Button::DOWN) {
            self.pos.y += (self.speed * DELTA) as i32
        }

        if controller.is_pressed(Button::LEFT) {
            self.pos.x -= (self.speed * DELTA) as i32;
        } else if controller.is_pressed(Button::RIGHT) {
            self.pos.x += (self.speed * DELTA) as i32;
        }
    }

    fn prevent_movement(&mut self) {
        self.pos = self.prev_pos;
    }

}

impl GameObj for Player {
    fn update(&mut self, controller: &ButtonController) {
        self.prev_pos = self.pos;
        self.pos.x = self.pos.x.clamp(0, agb::display::WIDTH - 32);
        self.pos.y = self.pos.y.clamp(0, agb::display::HEIGHT - 32);
        self.handle_input(controller);
        self.object.set_pos(self.pos);
    }

    fn on_screen(&self) -> bool {
        return self.on_screen;
    }

    fn check_to_free(&self) -> bool {
        return self.free_ready;
    }

    fn check_response_type(&self) -> ResponseType {
        return ResponseType::PLAYER;
    }

    fn handle_response(&mut self, response: ResponseType) {
        match response {
            ResponseType::NONE => {
            },
            ResponseType::DAMAGE => {
            },
            ResponseType::WALL => {
                self.prevent_movement();
            },
            ResponseType::PLAYER => {
            },
        }
    }

    fn get_pos(&self) -> Option<Vector2D<i32>> {
        return Some(self.pos);
    }

    fn draw(&self, frame: &mut GraphicsFrame) {
        self.object.show(frame);
    }
}