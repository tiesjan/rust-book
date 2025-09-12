use std::cmp::Ordering;
// "use" seems to be different from importing, as `std::io::stdin` without `use` is valid too
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    // Inferred to be `u32` due to type annotation for `guess` and use in comparison
    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
