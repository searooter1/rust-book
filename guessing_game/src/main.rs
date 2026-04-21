use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the Number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        // :: indicates function associated with string
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if guess.trim() == "quit" || guess.trim() == "q" {
            break;
        }

        // guess.trim() returns the guess string removing some whitespace
        // .parse() is used on the result of .trim() to parse it into an u32
        // If the parse succeeds it returns an Ok value which is part of the Result enum
        // The Ok value contains the parsed value
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // This is comparing the values of guess and secret_number
        // cmp returns a value of the Ordering enum which is pattern matched with the match statement
        // to determine what the program does next
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
