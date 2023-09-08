use std::env;

mod io;
mod map;
use map::matrix::Axis::{X, Y};
use map::matrix::{Map, MapItem, Point};
use utils::parse_int;

use crate::map::matrix::parse_matrix;
pub mod utils;

fn validate_input(args: Vec<String>) -> Option<(String, String, usize, usize)> {
    if args.len() <= 2 {
        println!("ERROR: [No se ingreso un directorio de output].");
        return None;
    } else if args.len() < 4 {
        io::create_directory(&args[2]);
        io::write_error(
            &(args[2].to_string() + "/" + &args[1].to_string()),
            "ERROR: [No se ingresaron todos los argumentos].",
        );
        return None;
    }
    io::create_directory(&args[2]);

    let input_file = args[1].to_string();
    let output_file = args[2].to_string() + "/" + &input_file;

    let (x, y) = match (parse_int(&args[3]), parse_int(&args[4])) {
        (Ok(x), Ok(y)) => (x, y),
        _ => {
            io::write_error(
                &output_file,
                "ERROR: [Error al interpretar puntos de vida/alcance en el archivo de entrada].",
            );
            return None;
        }
    };

    return Some((input_file, output_file, x, y));
}

fn read_map(input_file: &str, point: Point) -> Result<Map, String> {
    let map_raw = io::read_file(&input_file)?;
    let map = parse_matrix(map_raw)?;

    let (x, y) = (point.x, point.y);

    if map.dim(X) < x || map.dim(Y) < y {
        return Err(String::from(
            "ERROR: [Las coordenadas a detonar no se encuentran en el mapa].",
        ));
    }

    return Ok(map);
}

fn execute_turn(map: &mut Map, point: Point) -> Result<&Map, String> {
    match map.at(&point) {
        MapItem::Bomb(_) | MapItem::PiercingBomb(_) => {}
        _ => {
            return Err(String::from(
                "ERROR: [Las coordenadas ingresadas no corresponden a ninguna bomba].",
            ));
        }
    }

    map.detonate_bomb(&point)?;
    return Ok(map);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let (input_file, output_file, x, y) = match validate_input(args) {
        Some((input_file, output_file, x, y)) => (input_file, output_file, x, y),
        None => return,
    };

    let point_to_detonate = Point { x, y };

    let mut map = match read_map(&input_file, point_to_detonate) {
        Ok(map) => map,
        Err(e) => {
            io::write_error(&output_file, &e);
            return;
        }
    };

    match execute_turn(&mut map, point_to_detonate) {
        Ok(new_map) => {
            println!("{:?}", new_map)
        }
        Err(e) => {
            io::write_error(&output_file, &e);
            return;
        }
    };
}
