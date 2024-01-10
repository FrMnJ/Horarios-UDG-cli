mod escuela;
mod requests_horarios;
mod constants;
mod cli;

fn main() {
    let data = requests_horarios::get_input();
    let all_horarios = requests_horarios::get_horarios(data);
    loop {
        clearscreen::clear().expect("Failed to clear screen");
        cli::display_menu(&all_horarios);
    }
}
