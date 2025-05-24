use rand::random_range;
use std::{cmp::Ordering, io};
fn main() {
    println!("Guess the Number");
    let secret_number = random_range(1..=100);
    // println!("The secreat number is:{secret_number}");
    loop {
        println!("Input Your guess");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed:{guess}");
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!(" Winner Winner Chicken Dinner! ");
                break;
            }
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
        }
    }
}
