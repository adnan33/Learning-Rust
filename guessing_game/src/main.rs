#![allow(unused,unused_variables,while_true)]

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    

    while true
    {  
        let mut flag= String::new();
        println!("Guess The Number between 1 to 30!.");
        println!("Please input a number:");
    
        let secret_number = rand::thread_rng().gen_range(1..=30);
    
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Read line failed.");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}");
    
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You WIN!!!");
                break;
        },
        }
        println!("the number was: {secret_number}");
        println!("Press enter to continue, 'q' to quit!");

        io::stdin()
            .read_line(&mut flag)
            .expect("Read line failed.");

        match flag.trim(){
            "q" => break,
            _ => continue,
        }
    }

    
}
