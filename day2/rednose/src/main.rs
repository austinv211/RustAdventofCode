use rednose::Runner;

fn main() {    
    if let Err(e) = Runner::run() {
        println!("Error running program!: {e}");
    }
}