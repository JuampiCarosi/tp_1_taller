use std::env;

mod detour;
mod direction;
mod game;
mod io;
mod map;
mod map_elements;
mod point;
mod utils;

use map::Map;
use point::Point;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (input_file, output_file, x, y) = match game::validate_input(args) {
        Some((input_file, output_file, x, y)) => (input_file, output_file, x, y),
        None => return,
    };

    let point_to_detonate = Point::new(x, y);

    let mut map = match Map::new(&input_file) {
        Ok(map) => map,
        Err(e) => {
            io::write_error(&output_file, &e);
            return;
        }
    };

    match game::execute_turn(&mut map, point_to_detonate) {
        Ok(new_map) => io::write_output(&output_file, new_map),
        Err(e) => {
            io::write_error(&output_file, &e);
            return;
        }
    };
}
