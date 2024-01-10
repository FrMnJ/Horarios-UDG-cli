use std::io;
use std::process;
use crate::escuela::materia;
use crate::escuela::profesor;
use clearscreen;
use std::io::prelude::*;

use super::escuela::horario::Horario;


fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn busqueda_clave(clave: String,horarios: &Vec<Horario>){
    horarios.iter().filter(|horario| horario.materia().clave().to_ascii_lowercase() == clave.to_ascii_lowercase() )
    .for_each(|horario|{
        println!("{}",horario);
    })
}

fn busqueda_materia(materia: String,horarios: &Vec<Horario>) {
    horarios.iter().filter(|horario| horario.materia().nombre().to_ascii_lowercase().contains(&materia.to_ascii_lowercase()) )
    .for_each(|horario|{
        println!("{}",horario);
    })
}

fn busqueda_profesor(profesor: String,horarios: &Vec<Horario>) {
    horarios.iter().filter(|horario| horario.profesor().nombre_completo().to_ascii_lowercase().contains(&profesor.to_ascii_lowercase()) )
    .for_each(|horario|{
        println!("{}",horario);
    })
}

fn get_input_formatted(msg: &str)->String{
    let mut buf = String::new();
    println!("{msg}");
    io::stdin().read_line(&mut buf)
    .expect("Error leyendo opcion en menu principal");

    buf.trim().to_string()
}

fn mostrar_todos(horarios: &Vec<Horario>){
    println!("Total: {}",horarios.len());
    horarios.iter().for_each(|horario|{
        println!("{}",horario);
    })
}

fn busqueda_nrc(nrc: String, horarios: &Vec<Horario>){
    horarios.iter().filter(|horario| horario.nrc().to_ascii_lowercase() == nrc.to_ascii_lowercase() )
    .for_each(|horario|{
        println!("{}",horario);
    })
}

pub fn display_menu(horarios: &Vec<Horario>){
    println!("1) Busqueda por clave.");
    println!("2) Busqueda por nombre de materia.");
    println!("3) Busqueda por nombre de profesor.");
    println!("4) Mostrar todos.");
    println!("5) Busqueda por NRC.");
    println!("0) Salir");
    let buf = get_input_formatted("Ingresa una opción: ");
    
    // Check if the input is empty
    if buf.trim().is_empty() {
        println!("Error: La opción no puede estar vacía.");
        return;
    }

    let op: u32 = match buf.trim().parse() {
        Ok(parsed_op) => parsed_op,
        Err(e) => {
            println!("Error convirtiendo opción: {}", e);
            return;
        }
    };

    match op {
        1=>{
            clearscreen::clear().expect("Failed to clear screen");
            let clave = get_input_formatted("Ingresa la clave de la materia: ");
            busqueda_clave(clave,horarios);
            pause();
            clearscreen::clear().expect("Failed to clear screen");
        },
        2=>{
            clearscreen::clear().expect("Failed to clear screen");
            let nombre_materia = get_input_formatted("Ingresa el nombre de la materia:  ");
            busqueda_materia(nombre_materia,horarios);
            pause();
            clearscreen::clear().expect("Failed to clear screen");
        },
        3=> {
            clearscreen::clear().expect("Failed to clear screen");
            let profesor = get_input_formatted("Ingresa el nombre del profesor:  ");
            busqueda_profesor(profesor,horarios);
            pause();
            clearscreen::clear().expect("Failed to clear screen");
        },
        4=>{
            clearscreen::clear().expect("Failed to clear screen");
            mostrar_todos(horarios);
            pause();
            clearscreen::clear().expect("Failed to clear screen");
        },
        5=>{
            clearscreen::clear().expect("Failed to clear screen");
            let nrc = get_input_formatted("Ingresa el NRC:  ");
            busqueda_nrc(nrc,horarios);
            pause();
            clearscreen::clear().expect("Failed to clear screen");
        }
        0=>process::exit(0),
        _=>process::exit(1),
    }
}