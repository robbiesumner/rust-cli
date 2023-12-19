use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    for arg in args[1..].iter() { // exclude first arg: binary location
        println!("{}", arg);
    }
}
