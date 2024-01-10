use core::fmt;

use super::materia::Materia;
use super::profesor::Profesor;

pub struct Horario {
    nrc: String,
    seccion: String,
    cupo: u8,
    disponible: u8,
    profesor: Profesor,
    materia: Materia,
    especificaciones:  Vec<(String,String,String,String)>,
}

impl fmt::Display for Horario{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut especificaciones = String::new();
        for (hora,dias,edificio,aula) in &self.especificaciones{
            especificaciones.push_str(&format!("({},{},{},{})\n",hora,dias,edificio,aula))
        }
        write!(f,"Clave: {}\nNombre: {}\nCreditos: {}\nNRC: <{}>\nSección: <{}>\nCupo: <{}>\nDisponible: <{}>\nProfesor: {}\n{}",
            self.materia.clave(),
            self.materia.nombre(),
            self.materia.creditos(),
            self.nrc,
            self.seccion,
            self.cupo,
            self.disponible,
            self.profesor().nombre_completo(),
            especificaciones,
        )
    }
}

impl Horario{
    pub fn new
            (nrc: String, 
            seccion: String, 
            cupo: u8, 
            disponible: u8, 
            profesor: Profesor, 
            materia: Materia,
            especificaciones: Vec<(String,String,String,String)>
            ) -> Self
        {
            Horario{
                nrc,
                seccion,
                cupo,
                disponible,
                profesor,
                materia,
                especificaciones,
            }
        }
    
    pub fn nrc(&self) -> String{
        format!("{}",self.nrc)
    }

    pub fn set_nrc(&mut self,nrc: String){
        self.nrc = nrc;
    }

    pub fn seccion(&self) -> String{
        format!("{}",self.seccion)
    }

    pub fn set_seccion(&mut self, seccion: String){
        self.seccion = seccion;
    }

    pub fn cupo(&self) -> u8{
        self.disponible
    }

    pub fn profesor(&self) -> &Profesor{
        &self.profesor
    }

    pub fn materia(&self) -> &Materia{
        &self.materia
    }

    pub fn horarios(&self) -> &Vec<(String,String,String,String)>{
        &self.especificaciones
    }

    pub fn set_horarios(&mut self, new_horarios: Vec<(String,String,String,String)>){
        self.especificaciones = new_horarios;
    }
}