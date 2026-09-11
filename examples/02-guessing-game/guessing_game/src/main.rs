use rand::Rng; // Trait: Packs behaviour
use std::cmp::Ordering; // Type: Packs data
use std::io;

// io: Input/Output library, io comes from the standard library known as std;
//
// Preludes: can be seen as a pattern to make using multiple types more convenient.

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // In Rust, the variables are immutable by default, but we can make them mutable by using the `mut` keyword.
    //
    // Immutable means that the value of the variable cannot be changed after it is assigned.

    loop {
        println!("==================");
        println!("Please input your guess");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        println!("You guessed: {guess}");

        // Shadowing
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                println!("[DEBUG] {}", guess);
                guess.clear();
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            // Has arms
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    // If the expect method is not called on the Result enum, a warning will be generated at compile time.

    // The & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.

    // Append Check:
    // println!("Enter again for append check!");
    // io::stdin()
    //     .read_line(&mut guess)
    //     .expect("Failed to read the line");
    //

    // Shadowing
    // guess is again created which is an unsigned 32 bit integer now.
    // Parse function return 'Result' type.
    // Printing Enums (just for fun!)
    // println!(
    //     "{:?} {:?} {:?}",
    //     Ordering::Equal,
    //     Ordering::Less,
    //     Ordering::Greater
    // );
}
