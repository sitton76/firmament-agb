use agb::fixnum::{vec2, Rect};
use agb::input::Button;
use agb::{display::GraphicsFrame, fixnum::Vector2D};
use agb::include_aseprite;
use agb::display::object::Object;
use alloc::boxed::Box;
use crate::game_obj::{GameObj, ResponseType};
use crate::{actor, global_data, DELTA};

include_aseprite!(
    mod sprites,
    "gfx/new_img.aseprite"
);

//consts related to where the screen scroll should end
const LEFT_EDGE: i32 = -120;
const RIGHT_EDGE: i32 = 120;
const UP_EDGE: i32 = 80;
const DOWN_EDGE: i32 = -14;

// consts related to where the screen scroll should start based on the players position
const LEFT_SCROLL: i32 = 15;
const RIGHT_SCROLL: i32 = 209;
const UP_SCROLL: i32 = 15;
const DOWN_SCROLL: i32 = 124;

pub(crate) struct Player {
    object: Object,
    col: Rect<i32>,
    prev_pos: Vector2D<i32>,
    off_screen_pos: Vector2D<i32>,
    moving: [bool; 4],
    speed: f32,
    free_ready: bool
}

impl Player {
    pub fn new(starting_pos: Vector2D<i32>) -> Player {
        Player {
            object: Object::new(sprites::TEST_PLAYER.sprite(0)),
            col: Rect { position: starting_pos, size: vec2(16, 16) },
            prev_pos: Vector2D { x: 0, y: 0 },
            off_screen_pos: Vector2D { x: 0, y: 0 },
            moving: [false, false, false, false],
            speed: 100.0,
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
            //globals.queue_scene_transition(crate::scene::SCENES::TestScene);
            let mut new_pos = self.col.position.clone();
            new_pos.x += 16;
            globals.spawn_queue(actor::Actor::AWall(new_pos));
        } else if controller.is_just_pressed(Button::B) {
            globals.queue_scene_transition(crate::scene::SCENES::Map001);
        }
    }

    fn prevent_movement(&mut self) {
        // Prevents movement when reacting to a wall, also offsets during a scroll to prevent clipping.
        let mut additional_pushback: Vector2D<i32> = Vector2D { x: 0, y: 0 };

        if self.moving[0] {
            additional_pushback.x -= 2;
        } else if self.moving[1] {
            additional_pushback.x += 2;
        }

        if self.moving[2] {
            additional_pushback.y -= 2;
        } else if self.moving[3] {
            additional_pushback.y += 2;
        }
        self.col.position = self.prev_pos - additional_pushback;
    }

    fn move_camera_offset(&mut self, globals: &mut global_data::GlobalData) {
        // Prevents the player from reaching the looping point of the map while allowing screen scrolling to trigger.
        let mut pos_offset = globals.get_camera_offset();
        if (self.col.position.x < LEFT_SCROLL) && (self.off_screen_pos.x > LEFT_EDGE) {
            pos_offset.x -= 1;
            self.off_screen_pos.x -= 1;
            self.moving[0] = true;
        } else if (self.col.position.x > RIGHT_SCROLL) && (self.off_screen_pos.x < RIGHT_EDGE) {
            pos_offset.x += 1;
            self.off_screen_pos.x += 1;
            self.moving[1] = true;
        }

        if (self.col.position.y < UP_SCROLL) && (self.off_screen_pos.y < UP_EDGE) {
            pos_offset.y -= 1;
            self.off_screen_pos.y += 1;
            self.moving[2] = true;
        } else if (self.col.position.y > DOWN_SCROLL) && (self.off_screen_pos.y > DOWN_EDGE) {
            pos_offset.y += 1;
            self.off_screen_pos.y -= 1;
            self.moving[3] = true;
        }
        globals.set_camera_offset(pos_offset);
    } 

}

impl GameObj for Player {
    fn update(&mut self, globals: &mut global_data::GlobalData) {
        self.moving[0] = false;
        self.moving[1] = false;
        self.moving[2] = false;
        self.moving[3] = false;
        self.prev_pos = self.col.position;
        self.col.position.x = self.col.position.x.clamp(0, (agb::display::WIDTH - 16) as i32);
        self.col.position.y = self.col.position.y.clamp(0, (agb::display::HEIGHT - 16) as i32);
        self.handle_input(globals);
        self.move_camera_offset(globals);
        self.col.position -= globals.get_camera_offset();
        self.object.set_pos(self.col.position);
    }

    fn on_screen(&self) -> bool {
        return true;
    }

    fn check_to_free(&self) -> bool {
        return self.free_ready;
    }

    fn check_heap(&self) -> Option<i32> {
        return Some(core::mem::size_of::<Player>() as i32)
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
                return self.check_response_type();
            },
            None => {
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