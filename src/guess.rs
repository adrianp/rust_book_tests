use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    let secret = rand::thread_rng().gen_range(1..=100);
    let mut tries = 0;

    println!("Guess the number!");

    loop {
        println!("Make a guess:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Invalid input");

        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input");
                continue;
            }
        };

        tries = tries + 1;

        println!("You guessed: {guess}");

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win in {tries} tries!");
                break;
            }
        }
    }
}
