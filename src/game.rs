use crate::{io, map, map_elements::Item, point::Point};

pub fn validate_input(args: Vec<String>) -> Option<(String, String, usize, usize)> {
    if args.len() <= 2 {
        println!("ERROR: [No se ingreso un directorio de output].");
        return None;
    } else if args.len() <= 4 {
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

    println!("args {:?}", args);
    println!("in file: {}", input_file);
    println!("out file: {}", output_file);

    let (x, y) = match (args[3].parse::<usize>(), args[4].parse::<usize>()) {
        (Ok(x), Ok(y)) => (x, y),
        _ => {
            io::write_error(
                &output_file,
                "ERROR: [Error al interpretar coordenadas de la bomba, por favor ingrese numeros o intente con un numero mas pequeño].",
            );
            return None;
        }
    };

    return Some((input_file, output_file, x, y));
}

pub fn execute_turn(map: &mut map::Map, point: Point) -> Result<&map::Map, String> {
    if !map.is_point_in_map(&point) {
        return Err(String::from(
            "ERROR: [Las coordenadas a detonar no se encuentran en el mapa].",
        ));
    }

    match map.at(&point) {
        Item::Bomb(_) | Item::PiercingBomb(_) => {}
        _ => {
            return Err(String::from(
                "ERROR: [Las coordenadas ingresadas no corresponden a ninguna bomba].",
            ));
        }
    }

    map.detonate_bomb(&point)?;
    return Ok(map);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_validate_input() -> Result<(), std::io::Error> {
        let args = vec![
            String::from(""),
            String::from("test_dir/input.txt"),
            String::from("test_dir/output"),
            String::from("1"),
            String::from("2"),
        ];
        let (input_file, output_file, x, y) = validate_input(args).unwrap();
        assert_eq!(input_file, "test_dir/input.txt");
        assert_eq!(output_file, "test_dir/output/test_dir/input.txt");
        assert_eq!(x, 1);
        assert_eq!(y, 2);

        let dir = fs::metadata("test_dir/output/");
        assert!(dir.is_ok());
        fs::remove_dir_all("test_dir/output")?;

        assert!(
            validate_input(vec![String::from(""), String::from("test_dir/input.txt")]).is_none()
        );
        assert!(validate_input(vec![
            String::from(""),
            String::from("test_dir/input.txt"),
            String::from("test_dir/output"),
        ])
        .is_none());

        let file = io::read_file("test_dir/output/test_dir/input.txt").unwrap();
        assert_eq!(file, "ERROR: [No se ingresaron todos los argumentos].");
        fs::remove_dir_all("test_dir/output")?;

        assert!(validate_input(vec![
            String::from(""),
            String::from("test_dir/input.txt"),
            String::from("test_dir/output"),
            String::from("a"),
            String::from("b"),
        ])
        .is_none());

        let file = io::read_file("test_dir/output/test_dir/input.txt").unwrap();
        assert_eq!(file, "ERROR: [Error al interpretar coordenadas de la bomba, por favor ingrese numeros o intente con un numero mas pequeño].");
        fs::remove_dir_all("test_dir/output")?;

        Ok(())
    }

    #[test]
    fn test_execute_turn() {
        let mut map = map::Map::new("test_dir/map.txt").unwrap();
        let point = Point::new(0, 0);
        let new_map = execute_turn(&mut map, point).unwrap();
        assert_eq!(new_map.at(&point), &Item::Empty);

        assert!(execute_turn(&mut map, Point::new(0, 0)).is_err());
        assert!(execute_turn(&mut map, Point::new(10000, 0)).is_err());
    }
}
