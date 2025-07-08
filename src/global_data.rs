#![allow(dead_code, unused_variables)]
use agb::{fixnum::Vector2D, input::ButtonController};

use crate::scene::SCENES;

pub(crate) struct GlobalData {
    flags: [bool; Flags::FlagMax as usize],
    next_scene: Option<SCENES>,
    cam_offset: Vector2D<i32>,
    input_controller: ButtonController
}

impl GlobalData {
    //Constructor
    pub fn new() -> GlobalData {
        GlobalData {
            flags: [false; Flags::FlagMax as usize],
            next_scene: None,
            cam_offset: Vector2D { x: 0, y: 0 },
            input_controller: ButtonController::new()
        }
    }

    pub fn queue_scene_transition(&mut self, new_scene: SCENES) {
        self.next_scene = Some(new_scene);
    }

    pub fn scene_change_queued(&mut self) -> Option<SCENES> {
        let buff = self.next_scene;
        self.next_scene = None;
        return buff;
    }

    pub fn get_input(&self) -> &ButtonController {
        return &self.input_controller;
    }

    pub fn update_input(&mut self) {
        self.input_controller.update();
    }

    //Flag functions
    pub fn check_flag(self, entry: Flags) -> bool {
        return self.flags[entry as usize]
    }

    pub fn set_flag(&mut self, entry: Flags, state: bool) {
        self.flags[entry as usize] = state
    }

    //Camera offset functions
    pub fn get_cam_offset(self) -> Vector2D<i32> {
        return self.cam_offset
    }

    pub fn set_cam_offset(&mut self, new_offset : Vector2D<i32>) {
        self.cam_offset = new_offset
    }

    pub fn reset_offset(&mut self) {
        self.cam_offset = Vector2D::new(0, 0)
    }
}

pub enum Flags {
    TestWallSwitch,
    UnimplementedFlag0,
    UnimplementedFlag1,
    FlagMax //This should always be last.
}