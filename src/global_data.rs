#![allow(dead_code, unused_variables)]

/*
    Global data about the game, each object has the chance to touch this data when running update()
    Contains info about flags, what scene is queued up, the camera offset, and the controller.
*/

use agb::display::GraphicsFrame;
use agb::fixnum::Rect;
use agb::{fixnum::Vector2D, input::ButtonController};
use alloc::vec::Vec;
use crate::game_obj::ResponseType;
use crate::maps::MapInfo;
use crate::{actor, maps};
use crate::scene::{SCENES, BACKGROUNDS};

pub enum GAMEMODE {
    PLAY,
    MENU
}

// BG stuff starts
use agb::{include_background_gfx};
use agb::display::tiled::VRAM_MANAGER;
include_background_gfx!(
    mod background,
    BG1 => deduplicate "gfx/background.aseprite",
    BG2 => deduplicate "gfx/background_2.aseprite",
);

use agb::display::{
    Priority,
    tiled::{RegularBackground, RegularBackgroundSize, TileFormat},
};
// BG stuff ends

pub(crate) struct GlobalData {
    flags: [bool; Flags::FlagMax as usize],
    next_scene: Option<SCENES>,
    cam_offset: Vector2D<i32>,
    input_controller: ButtonController,
    current_mode: GAMEMODE,
    spawn_queue: Vec<actor::Actor>,
    bg: RegularBackground,
    current_bg: Option<BACKGROUNDS>,
    level_layout: Option<MapInfo>,
}

impl GlobalData {
    //Constructor
    pub fn new() -> GlobalData {
        let new_bg = RegularBackground::new(
                Priority::P3,
                RegularBackgroundSize::Background32x32,
                TileFormat::FourBpp
        );
        VRAM_MANAGER.set_background_palettes(background::PALETTES);
        GlobalData {
            flags: [false; Flags::FlagMax as usize],
            next_scene: None,
            cam_offset: Vector2D { x: 0, y: 0 },
            input_controller: ButtonController::new(),
            current_mode: GAMEMODE::PLAY,
            spawn_queue: Vec::new(),
            bg: new_bg,
            current_bg: None,
            level_layout: None,
        }
    }

    pub fn spawn_queue(&mut self, actor_type: actor::Actor) {
        self.spawn_queue.push(actor_type);
    }

    pub fn get_spawn_queue(&self) -> Vec<actor::Actor> {
        return self.spawn_queue.clone();
    }

    pub fn clear_spawn_queue(&mut self) {
        self.spawn_queue.clear();
    }

    pub fn get_mode(&self) -> &GAMEMODE {
        return &self.current_mode;
    }

    pub fn set_mode(&mut self, new_mode: GAMEMODE) {
        self.current_mode = new_mode;
    }

    pub fn queue_bg_change(&mut self, new_bg: Option<BACKGROUNDS>) {
        self.current_bg = new_bg;
    }

    pub fn process_bg(&mut self, frame: &mut GraphicsFrame) {
        match &self.current_bg {
            Some(new_bg) => {
                match new_bg {
                    BACKGROUNDS::BgImg1 => {
                        self.bg.fill_with(&background::BG1);
                    },
                    BACKGROUNDS::BgImg2 => {
                        self.bg.fill_with(&background::BG2);
                    },
                }
                self.bg.show(frame);
            },
            None => { self.bg.show(frame); },
        }
        self.current_bg = None;
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

    pub fn load_level_layout(&mut self, new_scene: SCENES) {
        self.level_layout = Some(maps::make_map(new_scene));
    }

    pub fn check_level_col(&self, checker: Rect<i32>) -> ResponseType {
        match &self.level_layout {
            Some(level) => {
                return level.is_colliding(checker)
            },
            None => return ResponseType::NONE,
        }
    }

    pub fn level_offset(&mut self) {
        let offset = self.get_camera_offset();
        match &mut self.level_layout {
            Some(layout) => {
                layout.offset_tiles(offset);
            },
            None => {},
        }
    }

    //Flag functions
    pub fn check_flag(self, entry: Flags) -> bool {
        return self.flags[entry as usize]
    }

    pub fn set_flag(&mut self, entry: Flags, state: bool) {
        self.flags[entry as usize] = state
    }

    //Camera offset functions
    pub fn get_camera_offset(&self) -> Vector2D<i32> {
        return self.cam_offset;
    }

    pub fn set_camera_offset(&mut self, new_offset: Vector2D<i32>) {
        self.cam_offset = new_offset;
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