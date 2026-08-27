use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Guess the number!");

    println!("Please input a number between 1 and one million");
    let mut guess = String::new();
    let secret_number = rand::thread_rng().gen_range(1..=100);

    let bytes_read = io::stdin().read_line(&mut guess).expect("Failed to read line");

    if bytes_read > 0 {
        println!("Just kidding haha, please guess a number from 1-100");
    };

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}