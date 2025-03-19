use std::io;
use std::env;

fn main() {
    let mut input = String::new();
    println!("Enter a word: ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    println!("You entered: {}", input);

    let args: Vec<String> = env::args().collect();
    let input = args[1].clone();
    println!("You entered: {:?}", args);
    println!("You entered: {:?}", input);

    if input=="hello" {
        println!("Hello, World!");
    } else {
        println!("Goodbye, World!");
    }
}
