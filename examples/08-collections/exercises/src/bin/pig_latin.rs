/*
 * Convert strings into Pig Latin. The first consonant of eahc word is moved to the end of the word and `ay` is added, first becomes irst-fay. Words that start with a vowel have hay added to the end instead (apple becomes apple-hay). Keep in mind the details about UTF-8 encoding!
 */

use std::io;

fn main() {
    println!("Enter the string to be converted into pig latin");

    let mut word = String::new();
    io::stdin()
        .read_line(&mut word)
        .expect("Please enter a valid input");

    let word = word.trim(); // removes '\n'

    /*
     * Iterate over the word using .chars()
     */

    let first = word.chars().nth(0).unwrap();

    match first {
        'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => {
            let result = format!("{word}-hay");
            println!("Pig Latin: {result}");
        }
        _ => {
            let slice = &word[1..];
            let result = format!("{slice}-{first}ay");
            println!("Pig Latin: {result}");
        }
    }
}
