mod cli;
mod classify;
mod benchmark;
mod verify;
mod visualize;
mod evolve;
mod config;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let result = cli::run(&args);
    if let Err(e) = result {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}
