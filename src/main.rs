use std::env;
use std::process;
use mygrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("{}",err);
        process::exit(1);
    });

    println!("Searching for {}\nIn file {}\n", config.query, config.fname);

    if let Err(e) = mygrep::run(config) {
        eprintln!("Application error: {}",e);
        process::exit(1);
    }
}