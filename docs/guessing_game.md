## Programming a Guessing Game

Let's make a guessing game. Program generates random integer between 1 and 100. Program will compare the player's guess and tell if its higher, lower, or the same. 

### Setting it up

- Make a new project
```bash
cargo new guessing_name
cd guessing_name
```

### Processing a guess

- Ask for user input
- Process the input
- Check if input in right form.

```rust
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
```

- The `io` library comes from the standard library, known as `std`. Check the [standard library documentation](https://doc.rust-lang.org/std/prelude/index.html) 
- Prelude: the set of items Rust brings into scope by default of every program. 
- if a type is not in prelude, we bring that into scope using `use` statement. 

### Strong values with variables

```rust
let mut guess = String::new(); # mutable
let apples = 5 # immutable
```

Variables are immutable by default i.e., the value does not change of a variable once given a value. However, we can make a variable mutable using the `mut` keyword.




Check the [complete code](guessing_game/src/main.rs)
