pub struct Profesor {
    nombre_completo: String,
}

impl Profesor{
    pub fn new(nombre_completo: String) -> Self{
        Profesor{
            nombre_completo,
        }
    }

    pub fn nombre_completo(&self) -> String{
        format!("{}",self.nombre_completo)
    }

    pub fn set_nombre_completo(&mut self, nombre_completo: String){
        self.nombre_completo = nombre_completo;
    }
}
