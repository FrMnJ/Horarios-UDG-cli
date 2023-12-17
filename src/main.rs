mod escuela;
mod requests_horarios;
mod constants;

fn main() {
    let data = requests_horarios::get_input();
    let all_horarios = requests_horarios::get_horarios(data);
}
