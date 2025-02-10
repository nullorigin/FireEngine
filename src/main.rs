#[derive(Debug, Clone)]
struct ProgramInfo {
    version: String,
    name: String,
    about: String,
    intro: String,
}

fn get_program_info() -> ProgramInfo {
    ProgramInfo {
        version: String::from("0.1.0"),
        name: String::from("FireEngine"),
        about: String::from("FireEngine is written in Rust. \nThis program generates a firewall configuration based on a set of specified logfiles."),
        intro: String::from("Welcome to FireEngine!"),
    }
}
fn main() {
    println!("{}", get_program_info().intro);
    println!("{}", get_program_info().about);
    println!();
}