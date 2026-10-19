use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args = env::args();

    let config = Config::build(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1)
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(err) = minigrep::run(config) {
        eprintln!("Application error: {err}");
        process::exit(1)
    }
}
