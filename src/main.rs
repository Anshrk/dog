use std::fs::File;
use std::io::prelude::*;
use std::env;
fn main() {
    // set of all command line arguments
    let args: Vec<String> = env::args().collect();
    // looping through all commands
    for argument in args.iter(){
        // defautl argument is passed as the file location
        if argument==&args[0] {
            continue;
        }
        // open the file in the argument
        let mut file = File::open(&argument.as_str())
        .expect("Cannot Open the file");
        let mut contents = String::new();
        // add the contents of the file to a string
        file.read_to_string(&mut contents)
        .expect("Error reading the file");
        // print the content of the file
        println!("{}", contents);
    }
}