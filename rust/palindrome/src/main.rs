use std::io;

fn main() {
    println!("Check if a word is a palindrome");

    let mut raw_word = String::new();
        
    println!("Input your word here:");

    io::stdin()
        .read_line(&mut raw_word) // read input from terminal
        .expect("Failed to read input"); // throw an error, if occured

    raw_word = raw_word.trim().to_string();
    
    let word = raw_word.trim().to_uppercase(); // we need to trim the input from io

    if word == reverse_word(&word).to_uppercase() {
        println!("{} is a palindrome", raw_word)
    } else {
        println!("{} is not a palindrome", raw_word)
    }
}

fn reverse_word(word: &str) -> String {
    return word.chars().rev().collect::<String>();
}