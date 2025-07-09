use agb::fixnum::{vec2, Rect};
use agb::input::Button;
use agb::{display::GraphicsFrame, fixnum::Vector2D};
use agb::include_aseprite;
use agb::display::object::Object;
use alloc::boxed::Box;
use crate::game_obj::{GameObj, ResponseType};
use crate::{global_data, DELTA, DOWN_EDGE, LEFT_EDGE, RIGHT_EDGE, UP_EDGE};

include_aseprite!(
    mod sprites,
    "gfx/new_img.aseprite"
);

pub(crate) struct Player {
    object: Object,
    col: Rect<i32>,
    prev_pos: Vector2D<i32>,
    off_screen_pos: Vector2D<i32>,
    moving_up: bool,
    moving_down: bool,
    moving_left: bool,
    moving_right: bool,
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
            off_screen_pos: Vector2D { x: 0, y: 0 },
            moving_up: false,
            moving_down: false,
            moving_left: false,
            moving_right: false,
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
        // Prevents movement when reacting to a wall, also offsets during a scroll to prevent clipping.
        let mut additional_pushback: Vector2D<i32> = Vector2D { x: 0, y: 0 };
        if self.moving_up {
            additional_pushback.y -= 2;
        } else if self.moving_down {
            additional_pushback.y += 2;
        }

        if self.moving_left {
            additional_pushback.x -= 2;
        } else if self.moving_right {
            additional_pushback.x += 2;
        }
        self.col.position = self.prev_pos - additional_pushback;
    }

    fn move_camera_offset(&mut self, globals: &mut global_data::GlobalData) {
        // Prevents the player from reaching the looping point of the map while allowing screen scrolling to trigger.
        let mut pos_offset = globals.get_camera_offset();
        if (self.col.position.x < 15) && (self.off_screen_pos.x > LEFT_EDGE) {
            pos_offset.x -= 1;
            self.off_screen_pos.x -= 1;
            self.moving_left = true;
        } else if (self.col.position.x > 209) && (self.off_screen_pos.x < RIGHT_EDGE) {
            pos_offset.x += 1;
            self.off_screen_pos.x += 1;
            self.moving_right = true;
        }

        if (self.col.position.y < 15) && (self.off_screen_pos.y < UP_EDGE) {
            pos_offset.y -= 1;
            self.off_screen_pos.y += 1;
            self.moving_up = true;
        } else if (self.col.position.y > 124) && (self.off_screen_pos.y > DOWN_EDGE) {
            pos_offset.y += 1;
            self.off_screen_pos.y -= 1;
            self.moving_down = true;
        }
        globals.set_camera_offset(pos_offset);
    } 

}

impl GameObj for Player {
    fn update(&mut self, globals: &mut global_data::GlobalData) {
        self.moving_up = false;
        self.moving_down = false;
        self.moving_left = false;
        self.moving_right = false;
        self.prev_pos = self.col.position;
        self.col.position.x = self.col.position.x.clamp(0, agb::display::WIDTH - 16);
        self.col.position.y = self.col.position.y.clamp(0, agb::display::HEIGHT - 16);
        self.handle_input(globals);
        self.move_camera_offset(globals);
        self.col.position -= globals.get_camera_offset();
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
        match col_1.overlapping_rect(col_2) {
            Some(_) => {
                match other.check_response_type() {
                    ResponseType::WALL => self.prevent_movement(),
                    _ => { }, //Unhandled collision type
                }
                //col_2.position += cam_offset;
                return self.check_response_type();
            },
            None => {
                //col_2.position += cam_offset;
                return ResponseType::NONE;
            }
        }

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