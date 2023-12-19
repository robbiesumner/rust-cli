use std::env;
use std::fs;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    let output: String = args[1..].iter().map(|arg| {
        let content = fs::read_to_string(arg);
        
        // pattern matching
        match content {
            Ok(content) => content,
            
            // end program on error
            Err(e) => {
                println!("r-cat: {}: {}", arg, e);
                exit(1);
            }
        }
    }).collect();

    print!("{}", output);
}
