use super::materia::Materia;

pub struct Carrera{
    codigo_ciclo: String,
    nombre: String,
    materias: Vec<Materia>,
}


impl Carrera{
    pub fn new(codigo_ciclo: String, nombre: String) -> Self{
        Carrera{
            codigo_ciclo,
            nombre,
            materias: Vec::new(),
        }
    }

    pub fn codigo_ciclo(&self) -> String{
        format!("{}",self.codigo_ciclo)
    }

    pub fn set_codigo_ciclo(&mut self, codigo_ciclo: String){
        self.codigo_ciclo = codigo_ciclo;
    }

    pub fn nombre(&self) -> String{
        format!("{}",self.nombre)
    }

    pub fn set_nombre(&mut self,nombre: String){
        self.nombre = nombre;
    }
}