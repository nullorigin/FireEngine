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
        name: String::from("Firewall"),
        about: String::from("Firewall is a program written in Rust that generates a firewall configuration parsed from a set of specified logfiles."),
        intro: String::from("Welcome to Firewall!"),
    }
}
fn main() {
    println!("{}", get_info().intro);
    println!("{}", get_info().about);
}