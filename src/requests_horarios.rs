use std::collections::HashMap;
use std::io;
use reqwest;
use scraper::Selector;
use crate::constants::CENTROS;
use crate::escuela::horario::Horario;
use crate::escuela::materia::Materia;
use crate::escuela::profesor::Profesor;

use titlecase::titlecase;


pub fn get_horarios(data: HashMap<&str,String>) -> Vec<Horario> {
    let mut all_horarios: Vec<Horario> = Vec::new();
    let client = reqwest::blocking::Client::new(); 
    let response = client
        .post("http://consulta.siiau.udg.mx/wco/sspseca.consulta_oferta")
        .form(&data)
        .send()
        .expect("Error tratando de obtener horarios.");

    let document = scraper::Html::parse_document(response.text().expect("Error tratando de convertir respuesta a texto plano.").as_str());

    // Selectors
    let tddatos_selector = scraper::Selector::parse("tr[style=\"background-color:#FFFFFF;\"], tr[style=\"background-color:#e5e5e5;\"]").unwrap();
    let nrc_selector = scraper::Selector::parse("td.tddatos:nth-child(1)").unwrap();
    let clave_selector = scraper::Selector::parse("td.tddatos:nth-child(2)>a").unwrap();
    let nombre_selector = scraper::Selector::parse("td.tddatos:nth-child(3)>a").unwrap();
    let seccion_selector = scraper::Selector::parse("td.tddatos:nth-child(4)").unwrap();
    let creditos_selector = scraper::Selector::parse("td.tddatos:nth-child(5)").unwrap();
    let cupos_selector = scraper::Selector::parse("td.tddatos:nth-child(6)").unwrap();
    let disponibles_selector = scraper::Selector::parse("td.tddatos:nth-child(7)").unwrap();
    let profesor_selector = scraper::Selector::parse("td.tddatos:nth-child(9) > table > tbody > tr > td.tdprofesor:nth-child(2)").unwrap();
    let rows_horarios = scraper::Selector::parse("td[align=\"center\"] > table > tbody > tr").unwrap();
    let hora_selector = scraper::Selector::parse("td:nth-child(2)").unwrap();
    let dias_selector = scraper::Selector::parse("td:nth-child(3)").unwrap();
    let edificio_selector = scraper::Selector::parse("td:nth-child(4)").unwrap();
    let aula_selector = scraper::Selector::parse("td:nth-child(5)").unwrap();

    // Iterate over selected rows
    let _=document.select(&tddatos_selector)
        .for_each(|row| {
            let nrc: String = row.select(&nrc_selector).next().map_or("null".to_string(), |el| el.inner_html().trim().to_string());
            let clave: String = row.select(&clave_selector).next().map_or("null".to_string(), |el| el.inner_html().trim().to_string());
            let nombre_materia: String = row.select(&nombre_selector).next().map_or("null".to_string(), |el| el.inner_html().trim().to_string());
            let seccion: String = row.select(&seccion_selector).next().map_or("null".to_string(), |el| el.inner_html().trim().to_string());
            let creditos: String = row.select(&creditos_selector).next().map_or("null".to_string(), |el| el.inner_html().trim().to_string());
            let cupos: String = row.select(&cupos_selector).next().map_or("null".to_string(), |el| el.inner_html().trim().to_string());
            let disponibles: String = row.select(&disponibles_selector).next().map_or("null".to_string(), |el| el.inner_html().trim().to_string());
            let profesor: String = row.select(&profesor_selector).next().map_or("null".to_string(), |el| el.inner_html().trim().to_string());
            let profesor: String = titlecase(profesor.as_str());
            let mut horarios: Vec<(String,String,String,String)> = Vec::new();

            row.select(&rows_horarios)
            .for_each(|row_horario|{
                let hora = row_horario.select(&hora_selector).next().map_or("null".to_string(), |el| el.inner_html().trim().to_string());
                let dias = row_horario.select(&dias_selector).next().map_or("null".to_string(), |el| el.inner_html().trim().to_string());
                let edificio = row_horario.select(&edificio_selector).next().map_or("null".to_string(), |el| el.inner_html().trim().to_string());
                let aula = row_horario.select(&aula_selector).next().map_or("null".to_string(), |el| el.inner_html().trim().to_string());

                horarios.push((hora,dias,edificio,aula));
            }); 
            // Insertion to vec
            let profesor: Profesor = Profesor::new(profesor);
            let materia: Materia = Materia::new(clave, nombre_materia, creditos.trim().parse().expect("Error tratando de convertir creditos."));
            let horario: Horario = Horario::new(nrc, seccion, cupos.trim().parse().expect("Error tratando de convertir cupos."), disponibles.trim().parse().expect("Error tratando de convertir disponibles."), profesor, materia, horarios);
            all_horarios.push(horario);
        });

        all_horarios

}

fn get_majors(center_code: char) -> Result<HashMap<String,String>,reqwest::Error>{
    // Selectors
    let trtable_majors_selector: Selector = scraper::Selector::parse("table[border]>tbody>tr").unwrap();
    let code_major_selector: Selector = scraper::Selector::parse("td>a").unwrap();
    let name_major_selector: Selector = scraper::Selector::parse("td:nth-child(2)").unwrap();
    let mut majors_map: HashMap<String,String> = HashMap::new();
    // Define request
    let mut url_get_majors: String =  String::from("http://consulta.siiau.udg.mx/wco/sspseca.lista_carreras?cup=");
    url_get_majors.push(center_code);

    let body =  reqwest::blocking::get(url_get_majors)?
    .text()?;
    
    // Define scraper
    let document = scraper::Html::parse_document(body.as_str());
    let _= document.select(&trtable_majors_selector)
    .for_each(|row|{
        let code_major = row.select(&code_major_selector).next().map_or("null".to_string(), |el| el.inner_html().trim().to_string());
        let name_major = row.select(&name_major_selector).next().map_or("null".to_string(), |el| el.inner_html().trim().to_string());
        if code_major!="null" && name_major!="null" {
            majors_map.insert(code_major, name_major);
        }
    });

    Ok(majors_map)
}

pub fn get_input() -> HashMap<&'static str,String>{
    // options
    let mut majrp: String  = String::new();
    let mut cup : String = String::new();
    let mut crsep: String = String::new(); 
    let mut materiap: String = String::new();
    let mut horaip: String = String::new();
    let mut horafp: String = String::new();
    let mut edifp: String = String::new();
    let mut aulap: String = String::new();
    // Define request body
    let mut data: HashMap<&str, String> = std::collections::HashMap::new();
    let p_start = 0;
    data.insert("ciclop", "202410".to_string());
    for (_codigo_centro,titulo) in CENTROS{
        println!("{titulo}");
    };
    println!("Ingresa tu centro universitario: ");
    io::stdin().read_line(&mut cup)
    .expect("Error tratando de leer el codigo de centro.");

    let cup = cup.trim().to_string().to_uppercase();

    let majors_map = get_majors(cup.chars().next().expect("Codigo de centro invalido")).expect("Peticion de carreras invalida.");
    for (major_code,major_name) in majors_map{
        println!("{} - {}",major_code,major_name);
    }
    println!("Ingresa tu codigo de carrera: ");
    io::stdin().read_line(&mut majrp)
    .expect("Error tratando de leer codigo de carrera.");
    let majrp = majrp.trim().to_string().to_uppercase();
    data.insert("cup", cup);
    data.insert("majrp", majrp);
    data.insert("crsep", crsep);
    data.insert("materiap", materiap);
    data.insert("horaip", horaip);
    data.insert("horafp",horafp);
    data.insert("edifp", edifp);
    data.insert("aulap", aulap);
    data.insert("ordenp", "0".to_string());
    data.insert("mostrarp", "500".to_string());
    if p_start != 0{
        data.insert("p_start","100".to_string());
    }  
    data
}
