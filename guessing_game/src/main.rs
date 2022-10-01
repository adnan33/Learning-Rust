#![allow(unused,unused_variables)]

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    println!("Guess The Number between 1 to 30!.");
    println!("Please input a number:");

    let secret_number = rand::thread_rng().gen_range(1..=30);

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Read line failed.");
    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    println!("You guessed: {guess}");

    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too Small!"),
        Ordering::Greater => println!("Too Big"),
        Ordering::Equal => println!("You WIN!!!"),
    }
    println!("the number was: {secret_number}");
}
