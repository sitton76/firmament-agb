use core::iter::Map;

use agb::{fixnum::{vec2, Rect, Vector2D}, println};
use alloc::vec::Vec;

use crate::{game_obj::ResponseType, scene};

#[path = "maps/test_map.rs"] pub(crate) mod test_map;

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
        if self.current_row > self.width {
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
                        println!("pos {:?}", tile.pos);
                        return ResponseType::LEVEL;
                    },
                    None => { },
                }
            }
        }
        return ResponseType::NONE;
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