use std::io; // we need this library to do operation with input or output in terminal
use rand::Rng; // library that operate with random generator

const INFIMUM: u32 = 1;
const SUPREMUM: u32 =100;

fn main() {
    let secret_number = rand::thread_rng().gen_range(INFIMUM, SUPREMUM+1);

    // instantiate a variable (mutable) 'guess' of type string
    // String::new() -> function that returns a new instance of a String
    
    let mut int_guess: u32= 0;

    println!("Guess the number! between {} and {} inclusive", INFIMUM, SUPREMUM);

    while int_guess != secret_number{
        let mut guess = String::new();
        
        println!("Input your guess here:");

        io::stdin()
            .read_line(&mut guess) // read input from terminal
            .expect("Failed to read input"); // throw an error, if occur

        // allow user to keep input, ignore not number
        int_guess = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", int_guess);

        if int_guess < secret_number {
            println!("wrong answer, this is a hint for you:");
            println!("let's go higher!");
        } else {
            println!("wrong answer, this is a hint for you:");
            println!("Going a bit lower won't hurt..");
        }

    }
    println!("CONGRATULATION! You have guessed the secret number: {}", secret_number);
}
