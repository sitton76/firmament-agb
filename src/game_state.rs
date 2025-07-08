use agb::{display::GraphicsFrame, input::ButtonController};
use alloc::{boxed::Box, vec::Vec};
use crate::{game_obj::GameObj, global_data, scene_list, DELTA};

pub(crate) struct GameState {
    obj_box: Vec<Box<dyn GameObj>>,
    current_map: scene_list::SCENES,
    globals: global_data::GlobalData,
}

impl GameState {
    pub fn new() -> GameState {
        return GameState {
            obj_box: Vec::new(),
            current_map: scene_list::SCENES::TestScene,
            globals: global_data::GlobalData::new(),
        }
    }

    pub fn cycle_update(&mut self, frame: &mut GraphicsFrame) {
        match self.globals.scene_change_queued() {
            Some(new_scene) => self.change_scene(new_scene),
            None => {
                self.globals.update_input();
                update_free(&mut self.obj_box);
                update_objs(&mut self.obj_box, &mut self.globals);
                update_collisions(&mut self.obj_box);
                draw_objs(&mut self.obj_box, frame);
            },
        }

    }

    pub fn change_scene(&mut self, next_scene: scene_list::SCENES) {
        self.empty_box();
        let new_box = scene_list::get_layout(next_scene);
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