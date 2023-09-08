pub fn parse_int(raw_string: &str) -> Result<usize, String> {
    match raw_string.parse::<usize>() {
        Ok(n) => Ok(n),
        Err(_) => Err(String::from(
            "ERROR: [Error al interpretar puntos de vida/alcance en el archivo de entrada].",
        )),
    }
}
