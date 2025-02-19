use fireengine::base::{Config, FireEngine};

#[derive(Debug, Clone)]
struct CargoToml {
    name: String,
    version: String,
    edition: String,
    target: String,
    license: String,
    description: String,
    repository: String,
}

fn get_cargo_toml() -> CargoToml {
    CargoToml {
        name: String::from("fireengine"),
        version: String::from("0.1.0"),
        edition: String::from("2021"),
        target: String::from("x86_64-unknown-linux-gnu"),
        license: String::from("MIT"),
        description: String::from(
            "FireEngine generates a firewall configuration based on a set of specified logfiles.",
        ),
        repository: String::from("https://github.com/nullorigin/FireEngine"),
    }
}
fn print_cargo_toml() {
    let cargo_toml = get_cargo_toml();
    println!("[package]");
    println!("name = {}", cargo_toml.name);
    println!("version = {}", cargo_toml.version);
    println!("edition = {}", cargo_toml.edition);
    println!("target = {}", cargo_toml.target);
    println!("license = {}", cargo_toml.license);
    println!("description = {}", cargo_toml.description);
    println!("repository = {}", cargo_toml.repository);
    println!("\n[dependencies]");
}
pub(crate) mod fireengine;
fn main() {
    print_cargo_toml();
    let mut fe = FireEngine::new();
    println!();
    fe.daemon.debug_on();
    fe.daemon.config();
    fe.daemon.enable();
    fe.daemon.load();
    fe.daemon.start();
    fe.daemon.stop();
    fe.daemon.unload();
    fe.daemon.deconfig();
    fe.daemon.disable();
    fe.daemon.fail();
    fe.daemon.success();
    println!();
}
