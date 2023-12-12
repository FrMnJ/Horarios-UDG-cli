use core::num;

use reqwest;
use scraper::{Html, Selector};
use titlecase::titlecase;

mod escuela;

fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let mut data = std::collections::HashMap::new();
    let p_start = 0;
    data.insert("ciclop", "202410");
    data.insert("cup", "D");
    data.insert("majrp", "INNI");
    data.insert("crsep", "");
    data.insert("materiap", "");
    data.insert("horaip", "");
    data.insert("horafp", "");
    data.insert("edifp", "");
    data.insert("aulap", "");
    data.insert("ordenp", "0");
    data.insert("mostrarp", "100");
    if p_start != 0{
        data.insert("p_start","100");
    }   
    let response = client
        .post("http://consulta.siiau.udg.mx/wco/sspseca.consulta_oferta")
        .form(&data)
        .send()?;

    let document = scraper::Html::parse_document(response.text()?.as_str());

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
        .map(|row| {
            let nrc = row.select(&nrc_selector).next().map_or("null".to_string(), |el| el.inner_html().trim().to_string());
            let clave = row.select(&clave_selector).next().map_or("null".to_string(), |el| el.inner_html().trim().to_string());
            let nombre_materia = row.select(&nombre_selector).next().map_or("null".to_string(), |el| el.inner_html().trim().to_string());
            let seccion = row.select(&seccion_selector).next().map_or("null".to_string(), |el| el.inner_html().trim().to_string());
            let creditos = row.select(&creditos_selector).next().map_or("null".to_string(), |el| el.inner_html().trim().to_string());
            let cupos = row.select(&cupos_selector).next().map_or("null".to_string(), |el| el.inner_html().trim().to_string());
            let disponibles = row.select(&disponibles_selector).next().map_or("null".to_string(), |el| el.inner_html().trim().to_string());
            let profesor = row.select(&profesor_selector).next().map_or("null".to_string(), |el| el.inner_html().trim().to_string());
            let mut horarios: Vec<(String,String,String,String)> = Vec::new();

            row.select(&rows_horarios)
            .for_each(|row_horario|{
                let hora = row_horario.select(&hora_selector).next().map_or("null".to_string(), |el| el.inner_html().trim().to_string());
                let dias = row_horario.select(&dias_selector).next().map_or("null".to_string(), |el| el.inner_html().trim().to_string());
                let edificio = row_horario.select(&edificio_selector).next().map_or("null".to_string(), |el| el.inner_html().trim().to_string());
                let aula = row_horario.select(&aula_selector).next().map_or("null".to_string(), |el| el.inner_html().trim().to_string());

                horarios.push((hora,dias,edificio,aula));
            }); 

            println!("{},{},{},{},{},{},{},{}", nrc, clave, nombre_materia, seccion, creditos, cupos, disponibles, profesor);
            println!("Horarios: ");
            for horario in horarios{
                println!("({},{},{},{})",horario.0,horario.1,horario.2,horario.3);
            }
        })
        .collect::<Vec<_>>();

    Ok(())
}
