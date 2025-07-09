/*
    What is effectively the main loop for the program
    Each cycle of the loop ran in main will trigger cycle_update()
    Which will update all objects position, collision, and draw.
    Scene changes are handled in the globals struct.
*/

use agb::{display::GraphicsFrame, sound::mixer::Mixer};
use alloc::{boxed::Box, vec::Vec};
use crate::{game_obj::GameObj, global_data, scene};

pub(crate) struct GameState {
    obj_box: Vec<Box<dyn GameObj>>,
    current_map: scene::SCENES,
    globals: global_data::GlobalData,
}

impl GameState {
    pub fn new() -> GameState {
        return GameState {
            obj_box: Vec::new(),
            current_map: scene::SCENES::TestScene,
            globals: global_data::GlobalData::new()
        }
    }

    pub fn cycle_update(&mut self, frame: &mut GraphicsFrame) {
        match self.globals.scene_change_queued() {
            Some(new_scene) => {
                self.change_scene(new_scene);
                self.globals.reset_offset();
                self.globals.process_bg(frame);
            },
            None => {
                self.globals.update_input();
                self.globals.process_bg(frame);
                update_free(&mut self.obj_box);
                update_objs(&mut self.obj_box, &mut self.globals);
                update_collisions(&mut self.obj_box);
                draw_objs(&mut self.obj_box, frame);
                self.globals.reset_offset();
            },
        }
    }

    pub fn change_scene(&mut self, next_scene: scene::SCENES) {
        self.empty_box();
        let new_box = scene::get_layout(next_scene);
        self.globals.queue_bg_change(scene::get_bg_val(next_scene));
        for obj in new_box {
            self.add_obj(obj);
        }
        self.current_map = next_scene;
    }

    pub fn add_obj(&mut self, new_obj: Box<dyn GameObj>) {
        self.obj_box.push(new_obj);
        match self.obj_box.last_mut() {
            Some(val) => val.ready(),
            _ => return,
        }
    }
    
    pub fn empty_box(&mut self) {
        self.obj_box.clear();
        assert!(self.obj_box.is_empty());
    }
}

fn update_objs(obj_box: &mut Vec<Box<dyn GameObj>>, globals: &mut global_data::GlobalData) {
    for obj in obj_box {
        if obj.on_screen() {
            obj.update(globals);
        } else {
            obj.simple_update(globals);
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
                let reply = entry.check_collision(other);
                other.handle_response(reply);
            }
        }
    }
}