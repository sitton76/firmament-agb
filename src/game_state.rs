/*
    What is effectively the main loop for the program
    Each cycle of the loop ran in main will trigger cycle_update()
    Which will update all objects position, collision, and draw.
    Scene changes are handled in the globals struct.
*/

use agb::{display::GraphicsFrame, println};
use alloc::{boxed::Box, vec::Vec};
use crate::{actor, game_obj::GameObj, global_data, scene};

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
                // Logic for when changing scenes.
                self.change_scene(new_scene);
                self.globals.reset_offset();
                self.globals.process_bg(frame);
                self.globals.set_mode(global_data::GAMEMODE::PLAY);
            },
            None => {
                self.globals.update_input();
                match self.globals.get_mode() {
                    global_data::GAMEMODE::PLAY => {
                        // Main gameplay loop logic.
                        self.globals.process_bg(frame);
                        update_free(&mut self.obj_box);
                        self.spawn_objs_in_queue();
                        update_objs(&mut self.obj_box, &mut self.globals);
                        update_collisions(&mut self.obj_box);
                        check_for_level_col(&mut self.obj_box, &mut self.globals);
                        draw_objs(&mut self.obj_box, frame);
                        //_get_heap(&self.obj_box);
                        self.globals.reset_offset();
                    },
                    global_data::GAMEMODE::MENU => {
                        // Gameplay logic for while in menus.
                    },
                }

            },
        }
    }

    pub fn change_scene(&mut self, next_scene: scene::SCENES) {
        self.empty_box();
        let new_box = scene::get_layout(next_scene);
        self.globals.load_level_layout(next_scene);
        self.globals.queue_bg_change(scene::get_bg_val(next_scene));
        for obj in new_box {
            match self.add_obj(actor::spawn_actor(obj)) {
                Ok(_) => {},
                Err(err_msg) => println!("{}", err_msg),
            }
        }
        self.current_map = next_scene;
    }

    fn add_obj(&mut self, new_obj: Box<dyn GameObj>) -> Result<bool, &str> {
        if find_obj_slot(&mut self.obj_box) {
            self.obj_box.push(new_obj);
            match self.obj_box.last_mut() {
                Some(val) => {
                    val.ready();
                    return Ok(true);
                }
                _ => return Err("Unable to get mutable reference to added object!"),
            }
        }
        return Err("obj_box is full! Skipping added object");
    }
    
    fn empty_box(&mut self) {
        self.obj_box.clear();
        assert!(self.obj_box.is_empty());
    }

    fn spawn_objs_in_queue(&mut self) {
        for child_queue_entry in self.globals.get_spawn_queue() {
            match self.add_obj(actor::spawn_actor(child_queue_entry)) {
                Ok(_) => {} ,
                Err(err_msg) => println!("{}", err_msg),
            }
        }
        self.globals.clear_spawn_queue();
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

fn find_obj_slot(obj_box: &mut Vec<Box<dyn GameObj>>) -> bool {
    if obj_box.len() < 128 {
        // If has enough slots to spawn something, returns true to allow new object to be spawned
        return true;
    } else {
        // If all 128 slots are full, attempt to free a slot.
        let mut can_spawn = false;
        let mut iter_count = 0;
        for obj in &mut *obj_box {
            if obj.can_cleanup() {
                // Checks each object to find one that can be cleaned up if able.
                can_spawn = true;
                break;
            } else {
                iter_count += 1;
            }
        }
        if can_spawn {
            // If a object that is cleanup able is found, it is removed from the obj_box and its slot will be filled with the new one later.
            obj_box.remove(iter_count);
        }
        return can_spawn;
    }
}

fn _get_heap(obj_box: &Vec<Box<dyn GameObj>>) {
    let mut heap_count: i32 = 0;
    for obj in obj_box {
        match obj.check_heap() {
            Some(got_heap_usage) => heap_count += got_heap_usage,
            None => {},
        }
    }
    println!("Heap usage: {}/262144", heap_count);
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
                if left[i].on_screen() || right[0].on_screen() {
                    let entry = &mut left[i];
                    let other = &mut right[0];
                    let reply = entry.check_collision(other);
                    other.handle_response(reply);
                }

            }
        }
    }
}

fn check_for_level_col(obj_box: &mut Vec<Box<dyn GameObj>>, globals: &mut global_data::GlobalData) {
    for obj in obj_box {
        match obj.get_collider() {
            Some(col) => {
                obj.handle_response(globals.check_level_col(col));
            },
            None => continue,
        }
    }
}