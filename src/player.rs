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
    x_pos: i32,
    y_pos: i32,
    prev_pos: Vector2D<i32>,
    speed: f32,
    on_screen: bool,
    free_ready: bool
}

impl Player {
    pub fn new() -> Player {
        Player {
            object: Object::new(sprites::TEST_PLAYER.sprite(0)),
            x_pos: 0,
            y_pos: 0,
            prev_pos: Vector2D { x: 0, y: 0 },
            speed: 100.0,
            on_screen: true,
            free_ready: false
        }
    }

    fn handle_input(&mut self, controller: &ButtonController) {
        if controller.is_pressed(Button::UP) {
            self.y_pos -= (self.speed * DELTA) as i32;
        } else if controller.is_pressed(Button::DOWN) {
            self.y_pos += (self.speed * DELTA) as i32
        }

        if controller.is_pressed(Button::LEFT) {
            self.x_pos -= (self.speed * DELTA) as i32;
        } else if controller.is_pressed(Button::RIGHT) {
            self.x_pos += (self.speed * DELTA) as i32;
        }
    }

    fn prevent_movement(&mut self) {
        self.x_pos = self.prev_pos.x;
        self.y_pos = self.prev_pos.y;
    }

}

impl GameObj for Player {
    fn update(&mut self, controller: &ButtonController) {
        self.prev_pos.x = self.x_pos;
        self.prev_pos.y = self.y_pos;
        self.x_pos = self.x_pos.clamp(0, agb::display::WIDTH - 32);
        self.y_pos = self.y_pos.clamp(0, agb::display::HEIGHT - 32);
        self.handle_input(controller);
        self.object.set_pos((self.x_pos, self.y_pos));
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
        return Some(Vector2D { x: self.x_pos, y: self.y_pos});
    }

    fn draw(&self, frame: &mut GraphicsFrame) {
        self.object.show(frame);
    }
}