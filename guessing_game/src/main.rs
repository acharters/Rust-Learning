use std::io;
use rand::Rng;
fn main() {
    //generate secret number
    let secret_number = rand::thread_rng().gen_range(1, 101);
    //get input from the user 
    println!("Guess a number: ");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Couldn't read line!");
    println!("your number: {}", guess);
}
