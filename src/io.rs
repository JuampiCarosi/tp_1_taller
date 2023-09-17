use std::fs;

use crate::map::Map;

/// Crea un directorio en caso de que no exista.
/// * `output_dir` - Nombre del directorio a crear.
pub fn create_directory(output_dir: &str) {
    let dir = fs::metadata(output_dir);
    if dir.is_ok() {
        return;
    }

    println!("{}", output_dir);
    match fs::create_dir_all(output_dir) {
        Ok(_) => {}
        Err(e) => {
            println!("ERROR: [No se pudo crear el directorio -- {e}].");
        }
    };
}

/// Escribe un mensaje de error en un archivo siguiendo el formato de la catedra.
/// * `output_file` - Nombre del archivo de salida.
/// * `mensaje` - Mensaje de error a escribir en el archivo
pub fn write_error(output_file: &str, mensaje: &str) {
    let mut dir = output_file.split('/').collect::<Vec<&str>>();
    dir.pop();
    if !dir.is_empty() {
        create_directory(&dir.join("/"));
    }
    match fs::write(output_file, mensaje) {
        Ok(_) => {}
        Err(e) => {
            println!("ERROR: [{e}].");
        }
    };
}

/// Lee un archivo y devuelve su contenido.
/// * `input_file` - Nombre del archivo a leer.
/// * `Result<String, String>` - Contenido del archivo o mensaje de error.
pub fn read_file(input_file: &str) -> Result<String, String> {
    match fs::read_to_string(input_file) {
        Ok(s) => Ok(s),
        Err(_) => Err(String::from(
            "ERROR: [No se pudo leer el archivo de entrada].",
        )),
    }
}

/// Escribe el mapa en un archivo siguiendo el formato de la catedra.
/// * `output_file` - Nombre del archivo de salida.
/// * `map` - Mapa a escribir en el archivo.
/// * `Result<(), String>` - Mensaje de error en caso de que no se pueda escribir el archivo.
pub fn write_output(output_file: &str, map: &Map) {
    let mut dir = output_file.split('/').collect::<Vec<&str>>();
    dir.pop();
    if !dir.is_empty() {
        create_directory(&dir.join("/"));
    }
    match fs::write(output_file, map.to_string()) {
        Ok(_) => {}
        Err(e) => {
            println!("ERROR: [{e}].");
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file() -> Result<(), String> {
        let file = read_file("test_dir/empty.txt")?;
        assert_eq!(file, "");

        let file = read_file("test_dir/one_line.txt")?;
        assert_eq!(file, "1 2 3 4 5");

        let file = read_file("test_dir/multiple_lines.txt")?;
        assert_eq!(file, "1 2 3 4 5\n6 7 8 9 10\n11 12 13 14 15");

        Ok(())
    }

    #[test]
    fn test_write_output() -> Result<(), String> {
        let map = Map::new("test_dir/map.txt")?;
        write_output("test_dir/output.txt", &map);

        let input = read_file("test_dir/map.txt")?;
        let output = read_file("test_dir/output.txt")?;
        assert_eq!(input, output);

        match fs::remove_file("test_dir/output.txt") {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    #[test]
    fn test_create_directory() -> std::io::Result<()> {
        create_directory("test_dir/test_dir");
        let dir = fs::metadata("test_dir/test_dir");
        assert!(dir.is_ok());
        fs::remove_dir_all("test_dir/test_dir")?;

        Ok(())
    }

    #[test]
    fn test_write_error() -> Result<(), String> {
        write_error("test_dir/error.txt", "ERROR: [Error de prueba].");
        let file = read_file("test_dir/error.txt")?;
        assert_eq!(file, "ERROR: [Error de prueba].");

        Ok(())
    }
}
