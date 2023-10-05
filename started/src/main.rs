use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {secret_number}");

    println!("Please input your guess.");

    let mut guess: String = String::new();

    io::stdin()
        .read_line( &mut guess)
        .expect("Fail the read line.");

    println!("Your guess {guess}");

    let guess: u32 = guess.trim().parse().expect("Please type a number.");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too Small"),
        Ordering::Greater => println!("Too Big"),
        Ordering::Equal => println!("You Win"),
    }
}
