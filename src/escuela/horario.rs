use super::materia::Materia;
use super::profesor::Profesor;

pub struct Horario {
    nrc: String,
    seccion: String,
    hora_inicio: u8,
    hora_fin: u8,
    cupo: u8,
    disponible: u8,
    profesor: Profesor,
    materia: Materia,
}

impl Horario{
    pub fn new
            (nrc: String, 
            seccion: String, 
            hora_inicio: u8, 
            hora_fin: u8, 
            cupo: u8, 
            disponible: u8, 
            profesor: Profesor, 
            materia: Materia) -> Self
        {
            Horario{
                nrc,
                seccion,
                hora_inicio,
                hora_fin,
                cupo,
                disponible,
                profesor,
                materia,
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

    pub fn hora_inicio(&self) -> u8{
        self.hora_inicio
    }

    pub fn set_hora_inicio(&mut self, hora_inicio: u8){
        self.hora_inicio = hora_inicio;
    }

    pub fn hora_fin(&self) -> u8{
        self.hora_fin
    }

    pub fn set_hora_fin(&mut self, hora_fin: u8){
        self.hora_fin = hora_fin;
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
}