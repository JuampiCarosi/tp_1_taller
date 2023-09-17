use crate::{detour::Detour, utils::parse_greater_than_zero_u32};
use std::fmt;

#[derive(PartialEq, Debug)]
pub enum Item {
    Empty,
    Wall,
    Rock,
    Enemy(u32),
    Bomb(u32),
    PiercingBomb(u32),
    Detour(Detour),
}

impl Item {
    pub fn parse(string: &str) -> Result<Item, String> {
        match string {
            "_" => Ok(Item::Empty),
            "W" => Ok(Item::Wall),
            "R" => Ok(Item::Rock),
            d if d.starts_with('D') => {
                let detour = Detour::parse(string)?;
                Ok(Item::Detour(detour))
            }
            f if f.starts_with('F') => {
                let (_, health_raw) = f.split_at(1);
                let health = parse_greater_than_zero_u32(health_raw)?;
                if health > 3 {
                    return Err(String::from(
                        "ERROR: [La vida de los enemigos no puede ser mayor a 3].",
                    ));
                }
                Ok(Item::Enemy(health))
            }
            b if b.starts_with('B') => {
                let (_, reach_raw) = b.split_at(1);
                let reach = parse_greater_than_zero_u32(reach_raw)?;
                Ok(Item::Bomb(reach))
            }
            s if s.starts_with('S') => {
                let (_, reach_raw) = s.split_at(1);
                let reach = parse_greater_than_zero_u32(reach_raw)?;
                Ok(Item::PiercingBomb(reach))
            }
            char => Err("ERROR: [El archivo de entrada contiene un caracter invalido '".to_owned()
                    + char + "']."),
        }
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Item::Empty => write!(f, "_"),
            Item::Wall => write!(f, "W"),
            Item::Rock => write!(f, "R"),
            Item::Enemy(h) => write!(f, "F{}", h),
            Item::Bomb(r) => write!(f, "B{}", r),
            Item::PiercingBomb(r) => write!(f, "S{}", r),
            Item::Detour(d) => write!(f, "{}", d),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_display() {
        assert_eq!(format!("{}", Item::Empty), "_");
        assert_eq!(format!("{}", Item::Wall), "W");
        assert_eq!(format!("{}", Item::Rock), "R");
        assert_eq!(format!("{}", Item::Enemy(4)), "F4");
        assert_eq!(format!("{}", Item::Bomb(4)), "B4");
        assert_eq!(format!("{}", Item::PiercingBomb(4)), "S4");
        assert_eq!(format!("{}", Item::Detour(Detour::Up)), "DU");
        assert_eq!(format!("{}", Item::Detour(Detour::Down)), "DD");
        assert_eq!(format!("{}", Item::Detour(Detour::Left)), "DL");
        assert_eq!(format!("{}", Item::Detour(Detour::Right)), "DR");
    }

    #[test]
    fn test_item_parse() -> Result<(), String> {
        assert_eq!(Item::parse("_")?, Item::Empty);
        assert_eq!(Item::parse("W")?, Item::Wall);
        assert_eq!(Item::parse("R")?, Item::Rock);
        assert_eq!(Item::parse("F3")?, Item::Enemy(3));
        assert_eq!(Item::parse("B4")?, Item::Bomb(4));
        assert_eq!(Item::parse("S4")?, Item::PiercingBomb(4));
        assert_eq!(Item::parse("DU")?, Item::Detour(Detour::Up));

        assert_eq!(
            Item::parse("F0"),
            Err(
                "ERROR: [El archivo de entrada contiene un numero menor o igual a cero]."
                    .to_string()
            )
        );

        assert_eq!(
            Item::parse("F4"),
            Err("ERROR: [La vida de los enemigos no puede ser mayor a 3].".to_string())
        );

        assert_eq!(
            Item::parse("H"),
            Err("ERROR: [El archivo de entrada contiene un caracter invalido 'H'].".to_string())
        );

        assert_eq!(
            Item::parse("Fb"),
            Err(
                "ERROR: [Error al interpretar puntos de vida/alcance en el archivo de entrada]."
                    .to_string()
            )
        );
        assert_eq!(
            Item::parse("F-1"),
            Err(
                "ERROR: [Error al interpretar puntos de vida/alcance en el archivo de entrada]."
                    .to_string()
            )
        );
        assert_eq!(
            Item::parse("D1"),
            Err(
                "ERROR: [El archivo de entrada contiene una direccion de desvio invalida]."
                    .to_string()
            )
        );

        Ok(())
    }
}
