extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    //generate secret number
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number); //for debugging
                                                         //loop until user guesses correctly
    loop {
        println!("Guess a number: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Couldn't read line!");
        let guess: i32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("This is not valid input. Try again.");
                continue;
            }
        };
        println!("your number: {}", guess);
        //check to see if the user's guess is too big or too small
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Guess is too small."),
            Ordering::Greater => println!("Guess is too big."),
            Ordering::Equal => {
                println!("This is the correct number!");
                break;
            }
        }
    }
}
