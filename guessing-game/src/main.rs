use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    let mut guessed_once = true;
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Guess a number between 1 and 1,000,000!");

    loop {
        let mut guess = String::new();

        let bytes_read = io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if bytes_read > 0 && guessed_once {
            println!("Just kidding haha, please guess a number between 1 and 100.");
            guessed_once = false;
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small! Guess again."),
            Ordering::Greater => println!("Too big! Guess again."),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}