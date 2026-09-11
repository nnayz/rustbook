# Rustbook

A short version of [The Rust Programming Language](https://doc.rust-lang.org/book/) ("The Book").

Each chapter is a condensed note in `docs/` plus a working crate you can run. This is a companion, not a substitute: when a page here is thin, go back to The Book.

Chapters 1–9 have notes. Later chapters are listed below so the path is visible; those files are not written yet.

## Follow along

You need `rustc` and `cargo` ([install with rustup](https://rustup.rs/)).

```bash
cd guessing_game
cargo run
```

Read the note first, then open the crate. Comments in `src/main.rs` are the same ideas in situ.

## Chapters

| # | Topic | Notes | Code |
|---|--------|--------|------|
| 1 | Getting Started | [01-getting-started.md](docs/01-getting-started.md) | [`hello_cargo/`](hello_cargo/), [`main.rs`](main.rs) |
| 2 | Programming a Guessing Game | [02-guessing-game.md](docs/02-guessing-game.md) | [`guessing_game/`](guessing_game/) |
| 3 | Common Programming Concepts | [03-common-concepts.md](docs/03-common-concepts.md) | [`variables/`](variables/), [`functions/`](functions/), [`control_flow/`](control_flow/) |
| 4 | Understanding Ownership | [04-ownership.md](docs/04-ownership.md) | [`ownership/`](ownership/), [`references_borrowing/`](references_borrowing/), [`slices/`](slices/) |
| 5 | Structs | [05-structs.md](docs/05-structs.md) | [`structs/`](structs/), [`methods/`](methods/) |
| 6 | Enums and Pattern Matching | [06-enums.md](docs/06-enums.md) | [`enums/`](enums/) |
| 7 | Packages, Crates, and Modules | [07-packages-crates-modules.md](docs/07-packages-crates-modules.md) | [`restaurant/`](restaurant/), [`backyard/`](backyard/), [`package_crates_modules/`](package_crates_modules/) |
| 8 | Common Collections | [08-collections.md](docs/08-collections.md) | [`collections/`](collections/), [`exercises/`](exercises/) |
| 9 | Error Handling | [09-error-handling.md](docs/09-error-handling.md) | [`error_handling/`](error_handling/) |
| 10 | Generic Types, Traits, and Lifetimes | [10-generics-traits-lifetimes.md](docs/10-generics-traits-lifetimes.md) — _not written yet_ | [`rustlings/`](rustlings/exercises/) `14_generics`, `15_traits`, `16_lifetimes`, `12_options` |
| 11 | Writing Automated Tests | [11-testing.md](docs/11-testing.md) — _not written yet_ | [`rustlings/`](rustlings/exercises/) `17_tests` |
| 12 | An I/O Project | [12-io-project.md](docs/12-io-project.md) — _not written yet_ | — |
| 13 | Iterators and Closures | [13-iterators-closures.md](docs/13-iterators-closures.md) — _not written yet_ | [`rustlings/`](rustlings/exercises/) `18_iterators` |
| 14 | More about Cargo and Crates.io | [14-cargo-crates-io.md](docs/14-cargo-crates-io.md) — _not written yet_ | — |
| 15 | Smart Pointers | [15-smart-pointers.md](docs/15-smart-pointers.md) — _not written yet_ | [`rustlings/`](rustlings/exercises/) `19_smart_pointers` |
| 16 | Fearless Concurrency | [16-concurrency.md](docs/16-concurrency.md) — _not written yet_ | [`rustlings/`](rustlings/exercises/) `20_threads` |
| 17 | Async / Await | [17-async.md](docs/17-async.md) — _not written yet_ | — |
| 18 | OOP Features of Rust | [18-oop.md](docs/18-oop.md) — _not written yet_ | — |
| 19 | Patterns and Matching | [19-patterns.md](docs/19-patterns.md) — _not written yet_ | — |
| 20 | Advanced Features | [20-advanced-features.md](docs/20-advanced-features.md) — _not written yet_ | [`rustlings/`](rustlings/exercises/) `21_macros` |
| 21 | Final Project: Web Server | [21-web-server.md](docs/21-web-server.md) — _not written yet_ | — |

Installation notes for Cargo/rustup: [docs/installation.md](docs/installation.md) — _not written yet_. Until then, use [rustup.rs](https://rustup.rs/).

The Book itself: [doc.rust-lang.org/book](https://doc.rust-lang.org/book/).

### 1. Getting Started

`fn main`, `println!`, `rustc`, then Cargo (`cargo new`, `build`, `run`, `check`, `Cargo.toml`).

- Notes: [docs/01-getting-started.md](docs/01-getting-started.md)
- [`main.rs`](main.rs) — hello world via `rustc main.rs`
- [`hello_cargo/`](hello_cargo/) — the same program as a Cargo project

### 2. Programming a Guessing Game

A first real program: stdin, `String`, `Result`/`expect`, the `rand` crate, `match`, shadowing, `loop`.

- Notes: [docs/02-guessing-game.md](docs/02-guessing-game.md)
- [`guessing_game/`](guessing_game/)

### 3. Common Programming Concepts

Mutability, shadowing, scalar and compound types, functions vs expressions, `if` / `loop` / `while` / `for`.

- Notes: [docs/03-common-concepts.md](docs/03-common-concepts.md)
- [`variables/`](variables/), [`functions/`](functions/), [`control_flow/`](control_flow/)
- Book exercises: [`temperature/`](temperature/), [`fibonacci/`](fibonacci/), [`twelve_days/`](twelve_days/)

### 4. Understanding Ownership

Stack vs heap, move / clone / copy, borrowing, slices (`&str`, `&[T]`).

- Notes: [docs/04-ownership.md](docs/04-ownership.md)
- [`ownership/`](ownership/), [`references_borrowing/`](references_borrowing/), [`slices/`](slices/)

### 5. Structs

Named fields, tuple structs, unit-like structs, `impl` methods, associated functions.

- Notes: [docs/05-structs.md](docs/05-structs.md)
- [`structs/`](structs/), [`methods/`](methods/)

### 6. Enums and Pattern Matching

Variants with data, `Option<T>`, exhaustive `match`, `if let`, `let...else`.

- Notes: [docs/06-enums.md](docs/06-enums.md)
- [`enums/`](enums/)

### 7. Packages, Crates, and Modules

Package vs crate vs module, `pub`, paths, `use`, splitting modules into files.

- Notes: [docs/07-packages-crates-modules.md](docs/07-packages-crates-modules.md)
- [`restaurant/`](restaurant/) — module tree in a library crate
- [`backyard/`](backyard/) — `mod` mapped to files
- [`package_crates_modules/`](package_crates_modules/) — extra notes in `src/doc.md`

### 8. Common Collections

`Vec<T>`, `String` / UTF-8, `HashMap<K, V>`.

- Notes: [docs/08-collections.md](docs/08-collections.md)
- [`collections/`](collections/)
- Book exercises in [`exercises/`](exercises/):

```bash
cd exercises
cargo run --bin mediam_mode        # median and mode
cargo run --bin pig_latin          # pig latin
cargo run --bin names_in_company   # departments
```

### 9. Error Handling

`panic!` vs `Result`, matching on `ErrorKind`.

- Notes: [docs/09-error-handling.md](docs/09-error-handling.md)
- [`error_handling/`](error_handling/)

### 10–21 (notes not written yet)

These files are the intended notes. Until they exist, The Book chapter and the rustlings exercises are the working path.

| Notes (planned) | The Book | Practice in this repo |
|-----------------|----------|------------------------|
| [10-generics-traits-lifetimes.md](docs/10-generics-traits-lifetimes.md) | [Ch. 10](https://doc.rust-lang.org/book/ch10-00-generics.html) | rustlings `12_options`, `14_generics`, `15_traits`, `16_lifetimes` |
| [11-testing.md](docs/11-testing.md) | [Ch. 11](https://doc.rust-lang.org/book/ch11-00-testing.html) | rustlings `17_tests` |
| [12-io-project.md](docs/12-io-project.md) | [Ch. 12](https://doc.rust-lang.org/book/ch12-00-an-io-project.html) | — |
| [13-iterators-closures.md](docs/13-iterators-closures.md) | [Ch. 13](https://doc.rust-lang.org/book/ch13-00-functional-features.html) | rustlings `18_iterators` |
| [14-cargo-crates-io.md](docs/14-cargo-crates-io.md) | [Ch. 14](https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html) | — |
| [15-smart-pointers.md](docs/15-smart-pointers.md) | [Ch. 15](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html) | rustlings `19_smart_pointers` |
| [16-concurrency.md](docs/16-concurrency.md) | [Ch. 16](https://doc.rust-lang.org/book/ch16-00-concurrency.html) | rustlings `20_threads` |
| [17-async.md](docs/17-async.md) | [Ch. 17](https://doc.rust-lang.org/book/ch17-00-async-await.html) | — |
| [18-oop.md](docs/18-oop.md) | [Ch. 18](https://doc.rust-lang.org/book/ch18-00-oop.html) | — |
| [19-patterns.md](docs/19-patterns.md) | [Ch. 19](https://doc.rust-lang.org/book/ch19-00-patterns.html) | — |
| [20-advanced-features.md](docs/20-advanced-features.md) | [Ch. 20](https://doc.rust-lang.org/book/ch20-00-advanced-features.html) | rustlings `21_macros` |
| [21-web-server.md](docs/21-web-server.md) | [Ch. 21](https://doc.rust-lang.org/book/ch21-00-final-project-a-web-server.html) | — |

## Extra practice: rustlings

[`rustlings/`](rustlings/) is a local copy of the [rustlings](https://github.com/rust-lang/rustlings) exercises. Mapping to The Book is in [`rustlings/exercises/README.md`](rustlings/exercises/README.md).

```bash
cd rustlings
cargo run --bin intro1
```

Solutions sit next to the exercises under `rustlings/solutions/`.

## Layout

```
docs/                  condensed notes, one file per Book chapter
hello_cargo/ …         one Cargo project per topic (chapters 1–9)
exercises/             chapter 8 binaries (median/mode, pig latin, company)
rustlings/             extra drills, including chapters that have no notes yet
main.rs                hello world, compiled with rustc
```

Each crate:

```bash
cd <crate>
cargo run
```
