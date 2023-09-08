pub mod matrix {
    use crate::utils::parse_int;
    use std::{collections::HashMap, fmt};

    #[derive(Clone, Copy)]
    pub enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    type Detour = Direction;

    impl fmt::Debug for Detour {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Detour::Up => write!(f, " ↑"),
                Detour::Down => write!(f, " ↓"),
                Detour::Left => write!(f, " ←"),
                Detour::Right => write!(f, " →"),
            }
        }
    }

    pub enum MapItem {
        Empty,
        Wall,
        Rock,
        Enemy(usize),
        Bomb(usize),
        PiercingBomb(usize),
        Detour(Detour),
    }

    #[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
    pub struct Point {
        pub x: usize,
        pub y: usize,
    }

    impl fmt::Debug for MapItem {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                MapItem::Empty => write!(f, "__"),
                MapItem::Wall => write!(f, " W"),
                MapItem::Rock => write!(f, " R"),
                MapItem::Enemy(h) => write!(f, "F{:?}", h),
                MapItem::Bomb(r) => write!(f, "B{}", r),
                MapItem::PiercingBomb(r) => write!(f, "S{}", r),
                MapItem::Detour(d) => write!(f, "{:?}", d),
            }
        }
    }

    impl fmt::Debug for Map {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "\n")?;
            for line in &self.0 {
                for item in line {
                    write!(f, "{:?}", item)?;
                }
                write!(f, "\n")?;
            }
            Ok(())
        }
    }

    pub enum Axis {
        X,
        Y,
    }

    impl Map {
        fn new() -> Map {
            Map(Vec::new())
        }

        fn push(&mut self, line: Vec<MapItem>) {
            self.0.push(line);
        }

        pub fn dim(&self, axis: Axis) -> usize {
            match axis {
                Axis::X => self.0.len(),
                Axis::Y => self.0[0].len(),
            }
        }

        pub fn at(&self, point: &Point) -> &MapItem {
            &self.0[point.y][point.x]
        }

        pub fn set_at(&mut self, point: &Point, item: MapItem) {
            self.0[point.y][point.x] = item;
        }

        pub fn get_explosion_properties(&self, point: &Point) -> Result<(usize, bool), String> {
            let bomb = self.at(&point);

            match bomb {
                MapItem::Bomb(reach) => Ok((*reach, false)),
                MapItem::PiercingBomb(reach) => Ok((*reach, true)),
                _ => Err(String::from(
                    "ERROR: [Ocurrio un error durante la ejecucion].",
                )),
            }
        }

        fn get_next_point(&self, current: &Point, direction: &Direction) -> Option<Point> {
            return match direction {
                Direction::Up => {
                    if current.y == 0 {
                        return None;
                    }
                    Some(Point {
                        x: current.x,
                        y: current.y - 1,
                    })
                }
                Direction::Down => {
                    if current.y == self.dim(Axis::Y) - 1 {
                        return None;
                    }
                    Some(Point {
                        x: current.x,
                        y: current.y + 1,
                    })
                }
                Direction::Left => {
                    if current.x == 0 {
                        return None;
                    }
                    Some(Point {
                        x: current.x - 1,
                        y: current.y,
                    })
                }

                Direction::Right => {
                    if current.x == self.dim(Axis::X) - 1 {
                        return None;
                    }
                    Some(Point {
                        x: current.x + 1,
                        y: current.y,
                    })
                }
            };
        }

        pub fn detonate_bomb(&mut self, point: &Point) -> Result<(), String> {
            let (reach, is_piercing) = self.get_explosion_properties(&point)?;
            self.set_at(&point, MapItem::Empty);

            self.spread_burst(&point, Direction::Up, is_piercing, reach)?;
            self.spread_burst(&point, Direction::Down, is_piercing, reach)?;
            self.spread_burst(&point, Direction::Left, is_piercing, reach)?;
            self.spread_burst(&point, Direction::Right, is_piercing, reach)?;
            Ok(())
        }

        fn spread_burst(
            &mut self,
            point: &Point,
            mut direction: Direction,
            is_piercing: bool,
            reach: usize,
        ) -> Result<(), String> {
            let mut current_point = *point;
            let mut enemies_to_damage: HashMap<Point, usize> = HashMap::new();
            for _ in 0..=reach {
                match self.at(&current_point) {
                    MapItem::Wall => break,
                    MapItem::Rock if !is_piercing => break,
                    MapItem::Bomb(_) | MapItem::PiercingBomb(_) => {
                        self.detonate_bomb(&current_point)?;
                        break;
                    }
                    MapItem::Detour(detour_direction) => {
                        direction = *detour_direction;
                    }
                    MapItem::Enemy(enemy) if !enemies_to_damage.contains_key(&current_point) => {
                        enemies_to_damage.insert(current_point, *enemy);
                    }
                    _ => {}
                };
                current_point = match self.get_next_point(&current_point, &direction) {
                    Some(p) => p,
                    None => break,
                };
            }

            for (enemy_point, enemy_health) in enemies_to_damage {
                let new_health = enemy_health - 1;
                if new_health == 0 {
                    self.set_at(&enemy_point, MapItem::Empty);
                } else {
                    self.set_at(&enemy_point, MapItem::Enemy(new_health));
                }
            }
            return Ok(());
        }
    }

    pub struct Map(Vec<Vec<MapItem>>);

    pub fn parse_matrix(raw_string: String) -> Result<Map, String> {
        let mut matrix = Map::new();

        let lines = raw_string.split("\n");
        for line in lines {
            let mut matrix_lines: Vec<MapItem> = Vec::new();
            let values = line.split(" ");

            for value in values {
                match value {
                    "_" => matrix_lines.push(MapItem::Empty),
                    "W" => matrix_lines.push(MapItem::Wall),
                    "R" => matrix_lines.push(MapItem::Rock),
                    d if d.starts_with("D") => {
                        let (_, direction) = d.split_at(1);
                        match direction {
                            "U" => matrix_lines.push(MapItem::Detour(Detour::Up)),
                            "D" => matrix_lines.push(MapItem::Detour(Detour::Down)),
                            "L" => matrix_lines.push(MapItem::Detour(Detour::Left)),
                            "R" => matrix_lines.push(MapItem::Detour(Detour::Right)),
                            _ => {
                                return Err(String::from(
                              "ERROR: [El archivo de entrada contiene una direccion de desvio invalida].",
                          ));
                            }
                        };
                    }
                    f if f.starts_with("F") => {
                        let (_, health_raw) = f.split_at(1);
                        let health = parse_int(health_raw)?;
                        matrix_lines.push(MapItem::Enemy(health));
                    }
                    b if b.starts_with("B") => {
                        let (_, reach_raw) = b.split_at(1);
                        let reach = parse_int(reach_raw)?;
                        matrix_lines.push(MapItem::Bomb(reach));
                    }
                    s if s.starts_with("S") => {
                        let (_, reach_raw) = s.split_at(1);
                        let reach = parse_int(reach_raw)?;
                        matrix_lines.push(MapItem::PiercingBomb(reach));
                    }
                    "" => {}
                    _ => {
                        return Err(String::from(
                            "ERROR: [El archivo de entrada contiene un caracter invalido].",
                        ));
                    }
                }
            }
            matrix.push(matrix_lines);
        }

        return Ok(matrix);
    }
}
