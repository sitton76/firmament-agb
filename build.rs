use std::fs;
use tiled::{Loader, PropertyValue};
use image::{ImageReader, Pixel};

fn main() {
    let map_list = ["test_map"];
    for map in map_list {
        convert_map(map.to_string());
    }
    let tile_list = ["tile_sheet"];
    for tile in tile_list {
        convert_tile_map(tile.to_string(), 16);
    }
}

fn convert_tile_map(map_name: String, tile_size: u32) {
    let map_path = format!("maps/tilesheets/{}.png", map_name);
    let out_path = format!("src/maps/tilesheets/{}.rs", map_name);

    match ImageReader::open(map_path) {
        Ok(img_reader) => {
            match img_reader.decode() {
                Ok(img) => {
                    let max_height = img.height();
                    let max_width = img.width();
                    match img.as_rgba8() {
                        Some(rgba_data) => {
                            let mut out = String::new();
                            let max_tile_count = (max_height / tile_size) * (max_width / tile_size);
                            out.push_str("use crate::maps::COLOR;\n");
                            out.push_str(format!("const IMG_SIZE: i32 = {};\n", max_tile_count).as_str());
                            let mut tile_count = 0;
                            let mut column_offset = 0;
                            let mut row_offset = 0;
                            let mut tile_container: Vec<Vec<[u8; 4]>> = Vec::new();
                            while tile_count < max_tile_count {
                                let mut new_tile: Vec<[u8 ; 4]> = Vec::new();
                                for y in (row_offset * tile_size)..(tile_size + (row_offset * tile_size)) {
                                    if y < max_height {
                                        for x in (column_offset * tile_size)..(tile_size + (column_offset * tile_size)) {
                                            if x < max_width {
                                                let pixel_data = rgba_data.get_pixel(x, y).to_rgba();
                                                let converted = [
                                                    pixel_data.0[0],
                                                    pixel_data.0[1],
                                                    pixel_data.0[2],
                                                    pixel_data.0[3]
                                                ];
                                                new_tile.push(converted);
                                            } else {
                                                break;
                                            }
                                        }
                                    } else {
                                        break;
                                    }
                                }
                                
                                tile_container.push(new_tile);
                                
                                tile_count += 1;
                                column_offset += 1;
                                if column_offset == max_width / tile_size {
                                    column_offset = 0;
                                    row_offset += 1;
                                }
                            }
                            let mut iter_count = 0;
                            let tile_length = tile_container[0].len();
                            for entry in tile_container {
                                out.push_str(format!("const TILE_{} : [[u8; 4]; {}] = [\n", iter_count, tile_length).as_str());
                                for sub_entry in entry {
                                    out.push_str(format!("{:?}, ", sub_entry).as_str());
                                }
                                out.push_str("];\n");
                                iter_count += 1;
                            }
                            fs::write(out_path, out).unwrap();
                        },
                        None => {
                            println!("Cannot get RBA data");
                            return;
                        },
                    }
                },
                Err(_) => {
                    println!("Cannot get image data");
                    return;
                },
            }
        },
        Err(_) => {
            println!("Cannot get image reader");
            return;
        },
    }

}

fn convert_map(map_name: String) {
    let map_path = format!("maps/{}.tmx", map_name);
    //let out_path = Path::new(&std::env::var("OUT_DIR").unwrap()).join("map_data.rs");
    let out_path = format!("src/maps/{}.rs", map_name);

    let mut loader = Loader::new();
    let map = loader.load_tmx_map(map_path).unwrap();

    let layer = map.layers().find_map(|l| l.as_tile_layer()).unwrap();
    let width = layer.width().unwrap();
    let height = layer.height().unwrap();

    let mut out = String::new();
    out.push_str(&format!("pub const MAP_WIDTH: usize = {};\n", width));
    out.push_str(&format!("pub const MAP_HEIGHT: usize = {};\n", height));
    out.push_str("pub static MAP_DATA: [[i32; 2]; MAP_WIDTH * MAP_HEIGHT] = [\n");


    for y in 0..height {
        for x in 0..width {
            match layer.get_tile(x as i32, y as i32) {
                Some(tile_layer) => {
                    match &tile_layer.get_tile() {
                        Some(tile) => {
                            let properties = tile.properties["COLLISION"] == PropertyValue::BoolValue(true);
                            out.push_str(&format!("[{}, {}],", tile_layer.id(), properties as i32));
                        },
                        None => out.push_str(&format!("[{}, 0],", tile_layer.id())),
                    }
                },
                None => { println!("Unable to get tile")},
            }
            
        }
        out.push('\n');
    }
    out.push_str("];\n");

    fs::write(out_path, out).unwrap();
}
