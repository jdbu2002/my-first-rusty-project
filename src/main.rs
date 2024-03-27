mod exercises;

use std::{io::stdin, process::exit};

fn main() {
    println!("Select a exercise number");

    let console = stdin();
    let mut input = String::new();

    console
        .read_line(&mut input)
        .expect("Oh no, your console exploded!");

    let parsed_input = input.trim();

    match parsed_input {
        "1" => exercises::exercise1::hello_world(),
        _ => {
            println!("Bye bye!");
            exit(0);
        }
    }
}
