pub struct Materia {
    clave: String,
    nombre: String,
    creditos: u8,
}

impl Materia{
    pub fn new(clave: String, nombre: String, creditos: u8) -> Self{
        Materia{
            clave,
            nombre,
            creditos,
        }
    }

    pub fn clave(&self) -> String{
        format!("{}",self.clave)
    }

    pub fn nombre(&self) -> String{
        format!("{}",self.nombre)
    }

    pub fn creditos(&self) -> String{
        format!("{}",self.creditos)
    }

    pub fn set_nombre(&mut self,nombre: String){
        self.nombre = nombre;
    }

    pub fn set_clave(&mut self,clave: String){
        self.clave = clave;
    }

    pub fn set_creditos(&mut self, new_creditos: u8){
        self.creditos = new_creditos;
    }

    pub fn to_string(&self) -> String{
        format!("{}, {}\nCreditos: {}",self.clave,self.nombre,self.creditos)
    }
}