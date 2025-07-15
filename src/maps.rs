use agb::{fixnum::{vec2, Rect, Vector2D}};
use alloc::vec::Vec;

use crate::{game_obj::ResponseType, scene};

// Keep maps 24x15 tiles (16x16 sized tiles)

// Collision data
#[path = "maps/test_map.rs"] pub(crate) mod test_map;

// Tilesheets
#[path = "maps/tilesheets/tile_sheet.rs"] pub(crate) mod tile_sheet_test;

enum TILESHEETS {
    TileSheetTest,
}

struct COLOR {
    R: u8,
    G: u8,
    B: u8,
    A: u8
}

impl COLOR {
    fn new(rgba: [u8; 4]) -> COLOR {
        COLOR {
            R: rgba[0],
            G: rgba[1],
            B: rgba[2],
            A: rgba[3]
        }
    }
}

pub fn fetch_tile_pixel(sheet: TILESHEETS, tile_id: i32, pixel: i32) -> Option<COLOR> {
    match sheet {
        TILESHEETS::TileSheetTest => return Some(COLOR::new(tile_sheet_test::TILES[tile_id as usize][pixel as usize])),
    }
}

pub(crate) struct TileData {
    pos: Vector2D<i32>,
    tile_id: i32,
    collision: bool
}

impl TileData {
    fn new(new_pos: Vector2D<i32>, new_id: i32, col: bool) -> TileData {
        TileData { pos: new_pos, tile_id: new_id, collision: col }
    }
}

pub(crate) struct MapInfo {
    height: i32,
    width: i32,
    tile_size: i32,
    current_row: i32,
    current_column: i32,
    map_data: Vec<TileData>,
}

impl MapInfo {
    pub fn new(new_height: i32, new_width: i32, size: i32) -> MapInfo {
        MapInfo {
            height: new_height,
            width: new_width,
            current_row: 0,
            current_column: 0,
            tile_size: size,
            map_data: Vec::new()
        }
    }

    pub fn add_tile(&mut self, id: i32, col: i32) {
        let tile_pos = vec2(self.current_row * self.tile_size, self.current_column * self.tile_size);
        let mut has_col = false;
        if col == 1 {
            has_col = true;
        }
        self.map_data.push(TileData::new(tile_pos, id, has_col));
        self.current_row += 1;
        if self.current_row > self.width - 1 {
            self.current_row = 0;
            self.current_column += 1;
        }
    }

    pub fn is_colliding(&self, obj_col: Rect<i32>) -> ResponseType {
        for tile in &self.map_data {
            if tile.collision {
                let tile_col = Rect::new(tile.pos, vec2(self.tile_size, self.tile_size));
                match tile_col.overlapping_rect(obj_col) {
                    Some(_) => {
                        return ResponseType::LEVEL;
                    },
                    None => { },
                }
            }
        }
        return ResponseType::NONE;
    }

    pub fn offset_tiles(&mut self, offset_val: Vector2D<i32>) {
        for tile in &mut self.map_data {
            tile.pos -= offset_val;
        }
    }
}

pub fn make_map(map_scene: scene::SCENES) -> MapInfo {
    match map_scene {
        scene::SCENES::TestScene => {
            let mut new_map = MapInfo::new(test_map::MAP_HEIGHT as i32, test_map::MAP_WIDTH as i32, 16);
            for tiles in test_map::MAP_DATA {
                new_map.add_tile(tiles[0], tiles[1]);
            }
            return new_map;
        },
        scene::SCENES::Map001 => {
            let mut new_map = MapInfo::new(test_map::MAP_HEIGHT as i32, test_map::MAP_WIDTH as i32, 16);
            for tiles in test_map::MAP_DATA {
                new_map.add_tile(tiles[0], tiles[1]);
            }
            return new_map;
        },
    }
}