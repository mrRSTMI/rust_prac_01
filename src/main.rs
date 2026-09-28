use std::io::stdin;
use std::io::Read;

fn main() {
    println!("Hello, world!");
    println!("Enter a number: ");
    let mut input_number = String::new();
    stdin().read_line(&mut input_number).expect("Failed to read line");
    let input_number: i32 = input_number.trim().parse().expect("Failed to parse number");
    println!("You entered: {}", input_number);
    
}
