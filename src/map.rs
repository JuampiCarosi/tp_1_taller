use std::{collections::HashMap, fmt};

use crate::{direction::Direction, io, map_elements::Item, point::Point};

pub struct Map(Vec<Vec<Item>>);

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in &self.0 {
            let mut formatted_line = String::new();
            for item in line {
                formatted_line.push_str(&(item.to_string() + " "));
            }
            formatted_line.pop();
            write!(f, "{}\n", formatted_line)?;
        }
        Ok(())
    }
}

impl Map {
    fn push_row(&mut self, line: Vec<Item>) {
        self.0.push(line);
    }

    fn pop_row(&mut self) {
        self.0.pop();
    }

    pub fn is_point_in_map(&self, point: &Point) -> bool {
        point.x < self.0[0].len() && point.y < self.0.len()
    }

    pub fn at(&self, point: &Point) -> &Item {
        &self.0[point.y][point.x]
    }

    pub fn set_at(&mut self, point: &Point, item: Item) {
        self.0[point.y][point.x] = item;
    }

    pub fn get_explosion_properties(&self, point: &Point) -> Result<(u32, bool), String> {
        let bomb = self.at(&point);

        match bomb {
            Item::Bomb(reach) => Ok((*reach, false)),
            Item::PiercingBomb(reach) => Ok((*reach, true)),
            _ => Err(String::from(
                "ERROR: [Ocurrio un error durante la ejecucion].",
            )),
        }
    }

    fn get_next_point(&self, current: &Point, direction: &Direction) -> Option<Point> {
        let x = current.x as isize;
        let y = current.y as isize;

        let next_coordenates: (isize, isize) = match direction {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        };

        if next_coordenates.0 < 0 || next_coordenates.1 < 0 {
            return None;
        }

        let next_point = Point::new(next_coordenates.0 as usize, next_coordenates.1 as usize);

        if !self.is_point_in_map(&next_point) {
            return None;
        }

        Some(next_point)
    }

    fn damage_enemies(&mut self, enemies_to_damage: HashMap<Point, u32>) {
        for (enemy_point, enemy_health) in enemies_to_damage {
            let new_health = enemy_health - 1;
            if new_health == 0 {
                self.set_at(&enemy_point, Item::Empty);
            } else {
                self.set_at(&enemy_point, Item::Enemy(new_health));
            }
        }
    }

    pub fn detonate_bomb(&mut self, point: &Point) -> Result<(), String> {
        let (reach, is_piercing) = self.get_explosion_properties(&point)?;
        self.set_at(&point, Item::Empty);

        self.spread_burst(&point, Direction::Up, is_piercing, reach)?;
        self.spread_burst(&point, Direction::Down, is_piercing, reach)?;
        self.spread_burst(&point, Direction::Left, is_piercing, reach)?;
        self.spread_burst(&point, Direction::Right, is_piercing, reach)?;
        Ok(())
    }

    fn spread_burst(
        &mut self,
        point: &Point,
        direction: Direction,
        is_piercing: bool,
        reach: u32,
    ) -> Result<(), String> {
        let mut current_point = *point;
        let mut enemies_to_damage: HashMap<Point, u32> = HashMap::new();
        let mut direction_to_use = &direction;

        for _ in 0..=reach {
            match self.at(&current_point) {
                Item::Wall => break,
                Item::Rock if !is_piercing => break,
                Item::Bomb(_) | Item::PiercingBomb(_) => {
                    self.detonate_bomb(&current_point)?;
                    break;
                }
                Item::Detour(detour_direction) => {
                    direction_to_use = detour_direction;
                }
                Item::Enemy(enemy) if !enemies_to_damage.contains_key(&current_point) => {
                    enemies_to_damage.insert(current_point, *enemy);
                }
                _ => {}
            };
            current_point = match self.get_next_point(&current_point, direction_to_use) {
                Some(p) => p,
                None => break,
            };
        }
        self.damage_enemies(enemies_to_damage);
        return Ok(());
    }

    pub fn new(input_file: &str) -> Result<Map, String> {
        let map_raw = io::read_file(&input_file)?;
        let mut map = Map(Vec::new());

        let lines = map_raw.split("\n");
        for line in lines {
            let mut matrix_row: Vec<Item> = Vec::new();
            let values = line.split(" ");

            for value in values {
                if value == "" {
                    continue;
                }
                let item = Item::parse(value)?;
                matrix_row.push(item);
            }

            map.push_row(matrix_row);
        }
        map.pop_row();

        Ok(map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_display() {
        let mut map = Map(Vec::new());

        assert!(format!("{}", map).is_empty());

        map.push_row(vec![Item::Empty, Item::Wall, Item::Empty]);
        map.push_row(vec![Item::Empty, Item::Rock, Item::PiercingBomb(3)]);
        map.push_row(vec![Item::Enemy(2), Item::Empty, Item::Bomb(1)]);

        assert_eq!(format!("{}", map), "_ W _\n_ R S3\nF2 _ B1\n");
    }

    #[test]
    fn test_map_push_row() {
        let mut map = Map(Vec::new());
        map.push_row(vec![Item::Empty, Item::Empty, Item::Empty]);
        map.push_row(vec![Item::Empty, Item::Empty, Item::Empty]);
        map.push_row(vec![Item::Empty, Item::Empty, Item::Empty]);

        assert_eq!(map.0.len(), 3);
        assert_eq!(map.0[0].len(), 3);
        assert_eq!(map.0[1].len(), 3);
        assert_eq!(map.0[2].len(), 3);
    }

    #[test]
    fn test_map_pop_row() {
        let mut map = Map(Vec::new());
        map.push_row(vec![Item::Empty, Item::Empty, Item::Empty]);
        map.push_row(vec![Item::Empty, Item::Empty, Item::Empty]);
        map.push_row(vec![Item::Empty, Item::Empty, Item::Empty]);

        map.pop_row();

        assert_eq!(map.0.len(), 2);
        assert_eq!(map.0[0].len(), 3);
        assert_eq!(map.0[1].len(), 3);
    }

    #[test]
    fn test_map_is_point_on_map() {
        let mut map = Map(Vec::new());
        map.push_row(vec![Item::Empty, Item::Empty, Item::Empty]);
        map.push_row(vec![Item::Empty, Item::Empty, Item::Empty]);

        assert!(map.is_point_in_map(&Point::new(0, 0)));
        assert!(!map.is_point_in_map(&Point::new(1, 3)));
    }

    #[test]
    fn test_map_at() {
        let mut map = Map(Vec::new());
        map.push_row(vec![Item::Empty, Item::Empty, Item::Empty]);
        map.push_row(vec![Item::Empty, Item::Wall, Item::Empty]);
        map.push_row(vec![Item::Empty, Item::Empty, Item::Rock]);

        assert_eq!(map.at(&Point::new(0, 0)), &Item::Empty);
        assert_eq!(map.at(&Point::new(1, 1)), &Item::Wall);
        assert_eq!(map.at(&Point::new(2, 2)), &Item::Rock);
    }

    #[test]
    fn test_map_set_at() {
        let mut map = Map(Vec::new());
        map.push_row(vec![Item::Empty, Item::Empty, Item::Empty]);
        map.push_row(vec![Item::Empty, Item::Empty, Item::Empty]);
        map.push_row(vec![Item::Empty, Item::Empty, Item::Empty]);

        map.set_at(&Point::new(0, 0), Item::Wall);
        map.set_at(&Point::new(1, 1), Item::Rock);
        map.set_at(&Point::new(2, 2), Item::Enemy(1));

        assert_eq!(map.at(&Point::new(0, 0)), &Item::Wall);
        assert_eq!(map.at(&Point::new(1, 1)), &Item::Rock);
        assert_eq!(map.at(&Point::new(2, 2)), &Item::Enemy(1));
    }

    #[test]
    fn test_map_get_explosion_properties() {
        let mut map = Map(Vec::new());
        map.push_row(vec![Item::Bomb(1), Item::PiercingBomb(2), Item::Empty]);
        map.push_row(vec![Item::Empty, Item::Empty, Item::Empty]);
        map.push_row(vec![Item::Empty, Item::Empty, Item::Empty]);

        assert_eq!(
            map.get_explosion_properties(&Point::new(0, 0)),
            Ok((1, false))
        );
        assert_eq!(
            map.get_explosion_properties(&Point::new(1, 0)),
            Ok((2, true))
        );

        assert!(map.get_explosion_properties(&Point::new(2, 0)).is_err());
    }

    #[test]
    fn test_map_get_next_point_on_edges() {
        let mut map = Map(Vec::new());
        map.push_row(vec![Item::Empty, Item::Empty, Item::Empty]);
        map.push_row(vec![Item::Empty, Item::Empty, Item::Empty]);
        map.push_row(vec![Item::Empty, Item::Empty, Item::Empty]);

        assert_eq!(map.get_next_point(&Point::new(0, 0), &Direction::Up), None);
        assert_eq!(
            map.get_next_point(&Point::new(0, 0), &Direction::Left),
            None
        );
        assert_eq!(
            map.get_next_point(&Point::new(2, 2), &Direction::Down),
            None
        );
        assert_eq!(
            map.get_next_point(&Point::new(2, 2), &Direction::Right),
            None
        );
    }
    #[test]
    fn test_map_get_next_point_on_center() {
        let mut map = Map(Vec::new());
        map.push_row(vec![Item::Empty, Item::Empty, Item::Empty]);
        map.push_row(vec![Item::Empty, Item::Empty, Item::Empty]);
        map.push_row(vec![Item::Empty, Item::Empty, Item::Empty]);

        assert_eq!(
            map.get_next_point(&Point::new(0, 1), &Direction::Up),
            Some(Point::new(0, 0))
        );
        assert_eq!(
            map.get_next_point(&Point::new(1, 0), &Direction::Left),
            Some(Point::new(0, 0))
        );
        assert_eq!(
            map.get_next_point(&Point::new(0, 0), &Direction::Down),
            Some(Point::new(0, 1))
        );
        assert_eq!(
            map.get_next_point(&Point::new(0, 0), &Direction::Right),
            Some(Point::new(1, 0))
        );
    }

    #[test]
    fn test_map_damage_enemies() {
        let mut map = Map(Vec::new());
        map.push_row(vec![Item::Empty, Item::Empty, Item::Empty]);
        map.push_row(vec![Item::Empty, Item::Enemy(1), Item::Empty]);
        map.push_row(vec![Item::Empty, Item::Enemy(2), Item::Empty]);

        let mut enemies_to_damage: HashMap<Point, u32> = HashMap::new();
        enemies_to_damage.insert(Point::new(1, 1), 1);
        enemies_to_damage.insert(Point::new(1, 2), 2);

        map.damage_enemies(enemies_to_damage);

        assert_eq!(map.at(&Point::new(1, 1)), &Item::Empty);
        assert_eq!(map.at(&Point::new(1, 2)), &Item::Enemy(1));
    }

    #[test]
    fn test_map_spread_burst_simple() -> Result<(), String> {
        let mut map = Map(Vec::new());
        map.push_row(vec![Item::Empty, Item::Empty, Item::Empty]);
        map.push_row(vec![Item::Empty, Item::Enemy(1), Item::Empty]);
        map.push_row(vec![Item::Empty, Item::Enemy(2), Item::Enemy(2)]);

        map.spread_burst(&Point::new(1, 1), Direction::Up, false, 1)?;
        assert_eq!(map.at(&Point::new(1, 1)), &Item::Empty);
        assert_eq!(map.at(&Point::new(1, 2)), &Item::Enemy(2));

        map.spread_burst(&Point::new(1, 2), Direction::Up, false, 1)?;
        assert_eq!(map.at(&Point::new(1, 2)), &Item::Enemy(1));

        map.spread_burst(&Point::new(0, 2), Direction::Right, false, 1)?;
        assert_eq!(map.at(&Point::new(2, 2)), &Item::Enemy(2));

        map.spread_burst(&Point::new(0, 2), Direction::Right, false, 2)?;
        assert_eq!(map.at(&Point::new(2, 2)), &Item::Enemy(1));

        map.spread_burst(&Point::new(0, 2), Direction::Right, true, 2)?;
        assert_eq!(map.at(&Point::new(2, 2)), &Item::Empty);

        Ok(())
    }

    #[test]
    fn test_map_spread_burst_with_walls() -> Result<(), String> {
        let mut map = Map(Vec::new());
        map.push_row(vec![Item::Enemy(1), Item::Empty, Item::Empty]);
        map.push_row(vec![Item::Wall, Item::Empty, Item::Empty]);
        map.push_row(vec![Item::Empty, Item::Empty, Item::Empty]);

        map.spread_burst(&Point::new(0, 2), Direction::Up, false, 1)?;
        assert_eq!(map.at(&Point::new(0, 1)), &Item::Wall);

        map.spread_burst(&Point::new(0, 2), Direction::Up, false, 2)?;
        assert_eq!(map.at(&Point::new(0, 1)), &Item::Wall);
        assert_eq!(map.at(&Point::new(0, 0)), &Item::Enemy(1));

        map.spread_burst(&Point::new(0, 2), Direction::Up, true, 1)?;
        assert_eq!(map.at(&Point::new(0, 1)), &Item::Wall);
        assert_eq!(map.at(&Point::new(0, 0)), &Item::Enemy(1));

        Ok(())
    }

    #[test]
    fn test_map_spread_burst_with_rocks() -> Result<(), String> {
        let mut map = Map(Vec::new());
        map.push_row(vec![Item::Enemy(1), Item::Enemy(1), Item::Empty]);
        map.push_row(vec![Item::Rock, Item::Rock, Item::Empty]);
        map.push_row(vec![Item::Empty, Item::Empty, Item::Empty]);

        map.spread_burst(&Point::new(0, 2), Direction::Up, false, 1)?;
        assert_eq!(map.at(&Point::new(0, 1)), &Item::Rock);

        map.spread_burst(&Point::new(0, 2), Direction::Up, false, 2)?;
        assert_eq!(map.at(&Point::new(0, 1)), &Item::Rock);
        assert_eq!(map.at(&Point::new(0, 0)), &Item::Enemy(1));

        map.spread_burst(&Point::new(0, 2), Direction::Up, true, 1)?;
        assert_eq!(map.at(&Point::new(0, 1)), &Item::Rock);
        assert_eq!(map.at(&Point::new(0, 0)), &Item::Enemy(1));

        map.spread_burst(&Point::new(1, 2), Direction::Up, true, 2)?;
        assert_eq!(map.at(&Point::new(1, 1)), &Item::Rock);
        assert_eq!(map.at(&Point::new(1, 0)), &Item::Empty);

        Ok(())
    }

    #[test]
    fn test_map_spread_burst_with_detours() -> Result<(), String> {
        let mut map = Map(Vec::new());
        map.push_row(vec![Item::Enemy(1), Item::Empty, Item::Empty]);
        map.push_row(vec![
            Item::Detour(Direction::Right),
            Item::Enemy(1),
            Item::Empty,
        ]);
        map.push_row(vec![
            Item::Empty,
            Item::Enemy(3),
            Item::Detour(Direction::Left),
        ]);

        map.spread_burst(&Point::new(0, 2), Direction::Up, false, 1)?;
        assert_eq!(map.at(&Point::new(0, 1)), &Item::Detour(Direction::Right));
        assert_eq!(map.at(&Point::new(0, 0)), &Item::Enemy(1));
        assert_eq!(map.at(&Point::new(1, 1)), &Item::Enemy(1));

        map.spread_burst(&Point::new(0, 2), Direction::Up, false, 2)?;
        assert_eq!(map.at(&Point::new(0, 0)), &Item::Enemy(1));
        assert_eq!(map.at(&Point::new(1, 1)), &Item::Empty);

        map.spread_burst(&Point::new(0, 2), Direction::Right, true, 2)?;
        assert_eq!(map.at(&Point::new(1, 2)), &Item::Enemy(2));

        map.spread_burst(&Point::new(0, 2), Direction::Right, true, 3)?;
        assert_eq!(map.at(&Point::new(1, 2)), &Item::Enemy(1));

        Ok(())
    }

    #[test]
    fn test_map_spread_burst_with_bombs() -> Result<(), String> {
        let mut map = Map(Vec::new());
        map.push_row(vec![Item::Enemy(1), Item::Empty, Item::Empty]);
        map.push_row(vec![Item::Bomb(1), Item::Enemy(1), Item::Empty]);
        map.push_row(vec![Item::Empty, Item::Empty, Item::Empty]);

        map.spread_burst(&Point::new(0, 2), Direction::Up, false, 1)?;
        assert_eq!(map.at(&Point::new(0, 1)), &Item::Empty);
        assert_eq!(map.at(&Point::new(0, 0)), &Item::Empty);
        assert_eq!(map.at(&Point::new(1, 1)), &Item::Empty);

        Ok(())
    }

    #[test]
    fn test_map_detonate_bomb() -> Result<(), String> {
        let mut map = Map(Vec::new());
        map.push_row(vec![Item::Enemy(1), Item::Empty, Item::Empty]);
        map.push_row(vec![Item::Bomb(1), Item::Enemy(1), Item::Empty]);
        map.push_row(vec![Item::Empty, Item::Empty, Item::Empty]);

        map.detonate_bomb(&Point::new(0, 1))?;
        assert_eq!(map.at(&Point::new(0, 1)), &Item::Empty);
        assert_eq!(map.at(&Point::new(0, 0)), &Item::Empty);
        assert_eq!(map.at(&Point::new(1, 1)), &Item::Empty);

        match map.detonate_bomb(&Point::new(0, 1)) {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(err, "ERROR: [Ocurrio un error durante la ejecucion]."),
        }

        Ok(())
    }

    #[test]
    fn test_map_parse() -> Result<(), String> {
        let map = Map::new("test_dir/map_parse.txt")?;

        assert_eq!(map.at(&Point::new(0, 0)), &Item::Empty);
        assert_eq!(map.at(&Point::new(1, 0)), &Item::Enemy(1));
        assert_eq!(map.at(&Point::new(0, 1)), &Item::Rock);
        assert_eq!(map.at(&Point::new(1, 1)), &Item::PiercingBomb(3));

        Ok(())
    }

    #[test]
    fn test_map_parse_invalid() -> Result<(), String> {
        match Map::new("test_dir/map_invalid.txt") {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(
                err,
                "ERROR: [El archivo de entrada contiene un caracter invalido 'H']."
            ),
        }

        Ok(())
    }
}
