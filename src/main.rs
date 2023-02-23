use std::cmp::Ordering;
use std::io;

use rand::{thread_rng, Rng};

fn main() {
    println!("Guess the number between 1 to 10!");

    loop {
        // Exclusive range
        let mut rng = thread_rng();
        let secret_number = rng.gen_range(0..10);

        println!("Please input your guess!");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("you guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too big :)"),
            Ordering::Less => println!("Too small (:"),
            Ordering::Equal => {
                println!("You Win..!");
                break;
            }
        }
    }
}
