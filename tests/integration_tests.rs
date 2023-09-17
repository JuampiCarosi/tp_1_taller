use tp_1::{
    game::{self, validate_input},
    io,
    map::Map,
    point::Point,
};

#[test]
fn map_with_bomb_and_enemies() {
    let (input_file, output_file, x, y) = validate_input(vec![
        String::from(""),
        String::from("test_dir/map_with_bomb_and_enemies.txt"),
        String::from("test_dir/output"),
        String::from("2"),
        String::from("4"),
    ])
    .unwrap();

    let point_to_detonate = Point::new(x, y);

    let mut map = match Map::new(&input_file) {
        Ok(map) => map,
        Err(e) => {
            io::write_error(&output_file, &e);
            assert!(false);
            return;
        }
    };

    match game::execute_turn(&mut map, point_to_detonate) {
        Ok(new_map) => io::write_output(&output_file, new_map),
        Err(e) => {
            io::write_error(&output_file, &e);
            assert!(false);
            return;
        }
    };

    let file = io::read_file("test_dir/output/test_dir/map_with_bomb_and_enemies.txt").unwrap();
    println!("{}", file);
    assert_eq!(
        file,
        "_ _ _ _ _ _ _
_ W _ W _ W _
_ _ _ R F1 _ _
_ W _ W R W _
_ _ _ _ _ _ _
_ W _ W _ W _
_ _ _ _ _ _ B1
"
    );
}

#[test]
fn map_with_piercing_and_detour() {
    let (input_file, output_file, x, y) = validate_input(vec![
        String::from(""),
        String::from("test_dir/map_with_piercing_and_detour.txt"),
        String::from("test_dir/output"),
        String::from("0"),
        String::from("4"),
    ])
    .unwrap();

    let point_to_detonate = Point::new(x, y);

    let mut map = match Map::new(&input_file) {
        Ok(map) => map,
        Err(e) => {
            io::write_error(&output_file, &e);
            assert!(false);
            return;
        }
    };

    match game::execute_turn(&mut map, point_to_detonate) {
        Ok(new_map) => io::write_output(&output_file, new_map),
        Err(e) => {
            io::write_error(&output_file, &e);
            assert!(false);
            return;
        }
    };

    let file = io::read_file("test_dir/output/test_dir/map_with_piercing_and_detour.txt").unwrap();
    assert_eq!(
        file,
        "_ _ _ _ _ _ _
_ W _ W _ W _
_ R R R _ _ _
_ W _ W _ W _
_ _ _ _ DU _ _
_ W _ W _ W _
_ _ _ _ _ _ _
"
    );
}

#[test]
fn map_with_enemy_between_detour() {
    let (input_file, output_file, x, y) = validate_input(vec![
        String::from(""),
        String::from("test_dir/map_with_enemy_between_detour.txt"),
        String::from("test_dir/output"),
        String::from("0"),
        String::from("4"),
    ])
    .unwrap();

    let point_to_detonate = Point::new(x, y);

    let mut map = match Map::new(&input_file) {
        Ok(map) => map,
        Err(e) => {
            io::write_error(&output_file, &e);
            assert!(false);
            return;
        }
    };

    match game::execute_turn(&mut map, point_to_detonate) {
        Ok(new_map) => io::write_output(&output_file, new_map),
        Err(e) => {
            io::write_error(&output_file, &e);
            assert!(false);
            return;
        }
    };

    let file = io::read_file("test_dir/output/test_dir/map_with_enemy_between_detour.txt").unwrap();
    assert_eq!(
        file,
        "_ _ _ _ _ _ _
_ W _ W _ W _
_ _ _ _ _ _ _
_ W _ W _ W _
_ _ _ _ F1 DL _
_ W _ W _ W _
_ _ _ _ _ _ _
"
    );
}

#[test]
fn calling_with_no_coordenates() {
    validate_input(vec![
        String::from(""),
        String::from("test_dir/map.txt"),
        String::from("test_dir/output"),
    ]);

    let file = io::read_file("test_dir/output/test_dir/map.txt").unwrap();
    assert_eq!(file, "ERROR: [No se ingresaron todos los argumentos].");
}

#[test]
fn map_life_4() {
    let (input_file, _output_file, _x, _y) = validate_input(vec![
        String::from(""),
        String::from("test_dir/map_life_4.txt"),
        String::from("test_dir/output"),
        String::from("0"),
        String::from("4"),
    ])
    .unwrap();

    match Map::new(&input_file) {
        Ok(_) => assert!(false),
        Err(e) => {
            assert_eq!(
                e,
                "ERROR: [La vida de los enemigos no puede ser mayor a 3]."
            );
        }
    }
}

#[test]
fn coordenates_outside_map() {
    let (input_file, output_file, x, y) = validate_input(vec![
        String::from(""),
        String::from("test_dir/coordenates_outside_map.txt"),
        String::from("test_dir/output"),
        String::from("0"),
        String::from("100"),
    ])
    .unwrap();

    let point_to_detonate = Point::new(x, y);

    let mut map = match Map::new(&input_file) {
        Ok(map) => map,
        Err(e) => {
            io::write_error(&output_file, &e);
            assert!(false);
            return;
        }
    };

    match game::execute_turn(&mut map, point_to_detonate) {
        Ok(_) => assert!(false),
        Err(e) => {
            assert_eq!(
                e,
                "ERROR: [Las coordenadas a detonar no se encuentran en el mapa]."
            );
        }
    };
}
