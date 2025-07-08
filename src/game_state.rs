use agb::{display::GraphicsFrame, input::ButtonController};
use alloc::{boxed::Box, vec::Vec};
use crate::{game_obj::GameObj, DELTA};

pub(crate) struct GameState {
    obj_box: Vec<Box<dyn GameObj>>,
    input_controller: ButtonController,
}

impl GameState {
    pub fn new() -> GameState {
        return GameState {
            obj_box: Vec::new(),
            input_controller: ButtonController::new(),
        }
    }

    pub fn cycle_update(&mut self, frame: &mut GraphicsFrame) {
        self.input_controller.update();
        update_free(&mut self.obj_box);
        update_objs(&mut self.obj_box, &self.input_controller);
        update_collisions(&mut self.obj_box);
        draw_objs(&mut self.obj_box, frame);
    }

    pub fn add_obj(&mut self, new_obj: Box<dyn GameObj>) {
        self.obj_box.push(new_obj);
        match self.obj_box.last_mut() {
            Some(val) => { val.ready(); },
            _ => { return; },
        }
    }
    
    pub fn clear(&mut self) {
        self.obj_box.clear();
        assert!(self.obj_box.is_empty());
    }
}

fn update_objs(obj_box: &mut Vec<Box<dyn GameObj>>, input: &ButtonController) {
        for obj in obj_box {
            if obj.on_screen() {
                obj.update(input);
            }
        }
}

fn draw_objs(obj_box: &mut Vec<Box<dyn GameObj>>, frame:&mut GraphicsFrame) {
        for obj in obj_box {
            if obj.on_screen() {
                obj.draw(frame);
            }
        }
}

fn update_free(obj_box: &mut Vec<Box<dyn GameObj>>) {
    //Checks for objects queued for removal one at a time, then removes them, cycles until it reaches the end of the object box.
    let mut found_free: bool = false;
    let mut iter_count: usize = 0;
    loop {
        for entry in &mut *obj_box {
            if entry.check_to_free() == true {
                found_free = true;
                break;
            }
            iter_count += 1;
        }
        if found_free == true {
            obj_box.swap_remove(iter_count);
            found_free = false;
            iter_count = 0;
            continue;
        }
        break;
    }
}

fn update_collisions(obj_box: &mut Vec<Box<dyn GameObj>>) {
    let len = obj_box.len();
    if len < 2 {
        return;
    }
    for i in 0..len {
        for j in 0..len {
            if i != j {
                let (left, right) = obj_box.split_at_mut(j);
                if left.len() == 0 {
                    break;
                }
                let entry = &mut left[i];
                let other = &mut right[0];
                let reply = entry.check_collision(other, DELTA);
                other.handle_response(reply);
            }
        }
    }
}