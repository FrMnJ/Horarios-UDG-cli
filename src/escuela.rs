mod horario;
mod profesor;
mod materia;
mod carrera;

use carrera::Carrera;

struct Centro{
    nombre: String,
    codigo: char,
    carreras: Vec<Carrera>,
}


struct Red{
    centros: Vec<Centro>,
}
