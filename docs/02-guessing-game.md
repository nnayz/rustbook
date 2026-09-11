# 2. Programming a Guessing Game

The program picks a random integer between 1 and 100. You guess. It says too small, too big, or you win.

Working crate: [`guessing_game/`](../guessing_game/).

```bash
cd guessing_game
cargo run
```

## Set it up

```bash
cargo new guessing_game
cd guessing_game
```

## Process a guess

Ask for input, store it, check that it looks like a number.

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

`io` comes from the standard library, `std`. Types in the **prelude** are in scope automatically. Everything else needs a `use`. Docs: [std prelude](https://doc.rust-lang.org/std/prelude/index.html).

Without `use`, the same call is `std::io::stdin`.

## Store values in variables

```rust
let mut guess = String::new(); // mutable
let apples = 5;                // immutable
```

Variables are immutable by default. `mut` makes them mutable.

`::new` is an **associated function** on `String` — a function implemented on the type, not on an instance. It returns an empty `String`.

So `let mut guess = String::new();` is a mutable binding to a new, empty string.

## Receive user input

```rust
io::stdin()
    .read_line(&mut guess)
```

`stdin()` returns a `std::io::Stdin`. `read_line` appends the typed line into the string you pass.

`&mut guess` is a **mutable reference**. `&` means reference: several places can use the same data without copying it. References are immutable by default, so mutation needs `&mut`.

### Handle possible failure

`read_line` returns `Result`. `Result` is an enum with variants `Ok` and `Err`.

`expect` crashes the program if the value is `Err`, and prints the message you passed. Without `expect` the program still compiles, but you get a warning (the `Result` is unused).

`{}` in `println!` is a placeholder:

```rust
println!("You guessed: {guess}");

let x = 5;
let y = 10;
println!("x = {x} and y + 2 = {}", y + 2);
```

## Use a crate

A **crate** is a tree of Rust source files. This project is a **binary crate** (an executable). A **library crate** is code meant to be used by other programs, not run on its own.

Add `rand` in `Cargo.toml`:

```toml
[dependencies]
rand = "0.8.5"
```

`0.8.5` means `^0.8.5`: at least 0.8.5, below 0.9.0.

Then:

```rust
use rand::Rng;

let secret_number = rand::thread_rng().gen_range(1..=100);
```

`Rng` is a trait that packs the random-number behaviour. `thread_rng()` gives a local random-number generator. `1..=100` is an inclusive range.

## Compare the guess

Parse the string into a number, then compare with `cmp` and `Ordering`:

```rust
use std::cmp::Ordering;

let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => {
        println!("Please type a number!");
        continue;
    }
};

match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => {
        println!("You win!");
        break;
    }
}
```

`trim()` drops the newline `read_line` leaves on the string. `parse()` returns `Result`.

Reusing the name `guess` for the `u32` is **shadowing**: a new variable hides the old one. Useful when you want to convert a value in place without inventing a second name.

`match` has **arms**. Each arm is a pattern and the code to run if it matches. `Ordering` is an enum: `Less`, `Greater`, `Equal`.

Wrap the prompt + parse + compare in `loop { ... }` so the player can guess until they hit `Equal`, then `break`.

Invalid input is not a crash: the `Err` arm prints a hint and `continue`s to the next iteration.

## Full program

See [`guessing_game/src/main.rs`](../guessing_game/src/main.rs).
