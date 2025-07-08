use agb::{display::GraphicsFrame, fixnum::Vector2D};
use agb::include_aseprite;
use agb::display::object::Object;
use crate::game_obj::{GameObj, ResponseType};

include_aseprite!(
    mod sprites,
    "gfx/new_img.aseprite"
);

pub(crate) struct Player {
    object: Object,
    x_pos: i32,
    y_pos: i32,
    x_vel: i32,
    y_vel: i32,
    on_screen: bool,
    free_ready: bool
}

impl Player {
    pub fn new() -> Player {
        Player {
            object: Object::new(sprites::TEST_PLAYER.sprite(0)),
            x_pos: 0,
            y_pos: 0,
            x_vel: 0,
            y_vel: 0,
            on_screen: true,
            free_ready: false
        }
    }
}

impl GameObj for Player {
    fn ready(&mut self) { //Called after entering the object_box
        return;
    }

    fn update(&mut self) { //Updates the object
        self.x_pos = (self.x_pos + self.x_vel).clamp(0, agb::display::WIDTH - 16);
        self.y_pos = (self.y_pos + self.y_vel).clamp(0, agb::display::HEIGHT - 16);
        self.object.set_pos((self.x_pos, self.y_pos));
    }

    fn on_screen(&self) -> bool { //Check if a object is on screen or not.
        return self.on_screen;
    }

    fn check_to_free(&self) -> bool { //Checks to see if the object should be freed
        return self.free_ready;
    }

    fn check_response_type(&self) -> ResponseType { //Returns the response type the object uses.
        return ResponseType::PLAYER;
    }

    fn handle_response(&mut self, response: ResponseType) { //Handles the reaction from the object being checked in "check_collision()"
        match response {
            ResponseType::NONE => {
            },
            ResponseType::DAMAGE => {
            },
            ResponseType::WALL => {
            },
            ResponseType::PLAYER => {
            },
        }
    }

    fn get_pos(&self) -> Option<Vector2D<i32>> {  //Gets the position of the object.
        return Some((self.x_pos, self.y_pos).into());
    }

    fn draw(&self, frame: &mut GraphicsFrame) { //Draws the object
        self.object.show(frame);
    }
}