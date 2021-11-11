use rand::Rng;
use std::{cmp::Ordering, io::stdin};

fn main() {
    println!("Welcome to the guessing game");

    // Generate random number
    let secret_number: u8 = rand::thread_rng().gen_range(1..101);
    let mut guess: String;

    loop {
        println!("numberis: {}", secret_number);
        println!("Please input your guess (from 1 -100) :");

        guess = String::new();

        // Read input line
        stdin().read_line(&mut guess).expect("Couldn't read line");

        let guess: u8 = guess.trim().parse().expect("Please enter a number");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => println!("Too High!"),
        }
    }
}
