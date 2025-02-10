#[derive(Debug, Clone)]
struct info {
    version: String,
    name: String,
    about: String,
    intro: String,
}

fn get_info() -> info {
    info {
        version: String::from("0.1.0"),
        name: String::from("FireEngine"),
        about: String::from("FireEngine is written in Rust. \nThis program generates a firewall configuration based on a set of specified logfiles."),
        intro: String::from("Welcome to FireEngine!"),
    }
}
fn main() {
    println!("{}", get_info().intro);
    println!("{}", get_info().about);
    println!();
}