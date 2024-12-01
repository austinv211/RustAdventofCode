use std::env;
use std::process;
use historian_hysteria::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem getting arguments: {err}");
        process::exit(1);
    });
    
    if let Err(e) = historian_hysteria::run(config) {
        println!("Error running program!: {e}");
    }

}