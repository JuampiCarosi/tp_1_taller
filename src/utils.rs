/// Parsea un string a un u32, si el numero es mayor a cero devuelve el numero, sino devuelve un string con el mensaje de error.
/// # Arguments
/// * `string` - String a parsear.
/// # Returns
/// * `Result<u32, String>` - Numero parseado o mensaje de error.
pub fn parse_greater_than_zero_u32(string: &str) -> Result<u32, String> {
    match string.parse::<u32>() {
        Ok(n) => {
            if n > 0 {
                Ok(n)
            } else {
                Err(String::from(
                    "ERROR: [El archivo de entrada contiene un numero menor o igual a cero].",
                ))
            }
        }
        Err(_) => Err(String::from(
            "ERROR: [Error al interpretar puntos de vida/alcance en el archivo de entrada].",
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_u32() -> Result<(), String> {
        let four = parse_greater_than_zero_u32("4")?;
        assert_eq!(4, four);

        let zero = parse_greater_than_zero_u32("0");
        assert!(zero.is_err());

        let minus_four = parse_greater_than_zero_u32("-4");
        assert!(minus_four.is_err());

        let not_a_number = parse_greater_than_zero_u32("not a number");
        assert!(not_a_number.is_err());

        Ok(())
    }
}
