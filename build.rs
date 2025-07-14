use std::fs;
use tiled::{Loader, PropertyValue};

fn main() {
    let map_list = ["test_map"];

    for map in map_list {
        convert_map(map.to_string());
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