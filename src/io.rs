use std::fs;

pub fn create_directory(output_dir: &str) {
    match fs::create_dir_all(output_dir) {
        Ok(_) => {}
        Err(e) => {
            println!("ERROR: [{e}].");
        }
    };
}

pub fn write_error(output_file: &str, mensaje: &str) {
    match fs::write(output_file, mensaje) {
        Ok(_) => {}
        Err(e) => {
            println!("ERROR: [{e}].");
        }
    };
}

pub fn read_file(input_file: &str) -> Result<String, String> {
    match fs::read_to_string(input_file) {
        Ok(s) => Ok(s),
        Err(_) => Err(String::from(
            "ERROR: [No se pudo leer el archivo de entrada].",
        )),
    }
}
