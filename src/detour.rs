use std::fmt;

use crate::direction::Direction;

/// Los desvios se implementan como direcciones, ya que solo se puede desviar arriba, abajo, izquierda o derecha, igual que las direcciones.
/// Esto faicilita la implementacion de los desvios ya que se puede usar la misma logica que para las direcciones al propagar explosiones.
pub type Detour = Direction;

impl fmt::Display for Detour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Detour::Up => write!(f, "DU"),
            Detour::Down => write!(f, "DD"),
            Detour::Left => write!(f, "DL"),
            Detour::Right => write!(f, "DR"),
        }
    }
}

impl Detour {
    /// Parsea un string en un desvio
    /// # Arguments
    /// * `string` - String a parsear.
    /// # Returns
    /// * `Result<Detour, String>` - Desvio parseado o mensaje de error.
    pub fn parse(string: &str) -> Result<Detour, String> {
        let (_, direction) = string.split_at(1);
        match direction {
            "U" => Ok(Detour::Up),
            "D" => Ok(Detour::Down),
            "L" => Ok(Detour::Left),
            "R" => Ok(Detour::Right),
            _ => Err(String::from(
                "ERROR: [El archivo de entrada contiene una direccion de desvio invalida].",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detour_display() {
        assert_eq!(format!("{}", Detour::Up), "DU");
        assert_eq!(format!("{}", Detour::Down), "DD");
        assert_eq!(format!("{}", Detour::Left), "DL");
        assert_eq!(format!("{}", Detour::Right), "DR");
    }

    #[test]
    fn test_detour_parse() {
        assert_eq!(Detour::parse("DU").unwrap(), Detour::Up);
        assert_eq!(Detour::parse("DD").unwrap(), Detour::Down);
        assert_eq!(Detour::parse("DL").unwrap(), Detour::Left);
        assert_eq!(Detour::parse("DR").unwrap(), Detour::Right);

        let err = match Detour::parse("DZ") {
            Err(e) => e,
            _ => String::from(""),
        };

        assert_eq!(
            err,
            "ERROR: [El archivo de entrada contiene una direccion de desvio invalida].",
        );
    }
}
