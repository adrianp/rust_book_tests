use rand::Rng;
use std::cmp::Ordering;
use std::io;

#[derive(Debug)]
pub struct Guess {
    value: i32
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess should be between 1 and 100, got {}", value);
        }
        Guess {value}
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

#[test]
#[should_panic(expected="should be between")]
fn guess_too_high() {
    Guess::new(1000);
}

fn main() {
    let secret = rand::thread_rng().gen_range(1..=100);
    let mut tries = 0;

    println!("Guess the number!");

    loop {
        println!("Make a guess:");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Invalid input");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input");
                continue;
            }
        };

        let my_guess = Guess::new(guess);

        tries = tries + 1;

        println!("You guessed: {:?}", my_guess.value);

        match my_guess.value.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win in {tries} tries!");
                break;
            }
        }
    }
}
