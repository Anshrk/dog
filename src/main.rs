use std::fs::File;
use std::io::prelude::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    for argument in args.iter(){
        if argument==&args[0] {
            continue;
        }

        let mut file = File::open(&argument.as_str())
        .expect("Cannot Open the file");

        
        let mut contents = String::new();
        file.read_to_string(&mut contents)
        .expect("Error reading the file");

        println!("{}", contents);
    }
}