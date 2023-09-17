#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]

/// Representa un punto en el mapa, utilizamos usize debido a la naturaleza de la estructura Mapa, el mismo es representado como
/// un vector de vectores, entonces a la hora de acceder al elemento en el punto (x, y), simplemente sera el elemento del mapa en la posicion mapa[y][x].
/// # Attributes
/// * `x` - Coordenada x del punto.
/// * `y` - Coordenada y del punto
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    /// Crea un nuevo punto.
    /// # Arguments
    /// * `x` - Coordenada x del punto.
    /// * `y` - Coordenada y del punto.
    pub fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}
