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

pub(crate) struct Player {
    object: Object,
    col: Rect<i32>,
    prev_pos: Vector2D<i32>,
    speed: f32,
    on_screen: bool,
    free_ready: bool
}

impl Player {
    pub fn new(starting_pos: Vector2D<i32>) -> Player {
        Player {
            object: Object::new(sprites::TEST_PLAYER.sprite(0)),
            col: Rect { position: starting_pos, size: vec2(16, 16) },
            prev_pos: Vector2D { x: 0, y: 0 },
            speed: 100.0,
            on_screen: true,
            free_ready: false
        }
    }

    fn handle_input(&mut self, globals: &mut global_data::GlobalData) {
        let controller = globals.get_input();
        if controller.is_pressed(Button::UP) {
            self.col.position.y -= (self.speed * DELTA) as i32;
        } else if controller.is_pressed(Button::DOWN) {
            self.col.position.y += (self.speed * DELTA) as i32
        }

        if controller.is_pressed(Button::LEFT) {
            self.col.position.x -= (self.speed * DELTA) as i32;
        } else if controller.is_pressed(Button::RIGHT) {
            self.col.position.x += (self.speed * DELTA) as i32;
        }

        if controller.is_just_pressed(Button::A) {
            globals.queue_scene_transition(crate::scene::SCENES::TestScene);
        } else if controller.is_just_pressed(Button::B) {
            globals.queue_scene_transition(crate::scene::SCENES::Map001);
        }
    }

    fn prevent_movement(&mut self) {
        self.col.position = self.prev_pos;
    }

}

impl GameObj for Player {
    fn update(&mut self, globals: &mut global_data::GlobalData) {
        self.prev_pos = self.col.position;
        self.col.position.x = self.col.position.x.clamp(0, agb::display::WIDTH - 16);
        self.col.position.y = self.col.position.y.clamp(0, agb::display::HEIGHT - 16);
        self.handle_input(globals);
        self.col.position = self.col.position;
        self.object.set_pos(self.col.position);
    }

    fn on_screen(&self) -> bool {
        return self.on_screen;
    }

    fn check_to_free(&self) -> bool {
        return self.free_ready;
    }

    fn check_collision(&mut self, other: &Box<dyn GameObj>) -> ResponseType {
        let col_1 = match self.get_collider() {
            Some(col) => { col },
            _ => { return ResponseType::NONE; },
        };
        let col_2 = match other.get_collider() {
            Some(col) => { col },
            _ => { return ResponseType::NONE; },
        };
        let found_obj = col_1.touches(col_2);
        if found_obj {
            match other.check_response_type() {
                ResponseType::WALL => self.prevent_movement(),
                _ => { }, //Unhandled collision type
            }
            return self.check_response_type();
        }
        return ResponseType::NONE;
    }

    fn check_response_type(&self) -> ResponseType {
        return ResponseType::PLAYER;
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