#![allow(unused,unused_variables)]

use std::io;
fn main() {
    println!("Guess The Number!.");
    println!("Please input a number:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Read line failed.");

    println!("You guessed: {guess}");
}
