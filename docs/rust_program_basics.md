## Rust Program Basics

- They always start with the `.rs` extension. 
- Using more than one word in the file name requires the convention to use underscore to separate them (Snake Case).
- Rust `hello_world.rs` example. 
```rust
fn main() {
    println!("Hello, world!");
}
```
- Running the program requires `rustc` (Rust Compiler) which takes the code (.rs) and cmpiles it into the binary executable. 
```bash
# This will create the executable
rustc main.rs

# This will run the executable
./main
```

### The Anatomy of a Rust Program

```rust
fn main() {
    println!("Hello, world!");
}
```

- Using a `!` means that we are calling a macro instead of a normal function. 
- Code inside the `main` function always runs first in the binary.
- Function body is wrapped in `{}`. Rust requires function body to be wrapped in these braces. 
- `println!` calls a Rust macro. 
- Rust macros are a way to write code that generates code to extend Rust syntax. (More about them in [Chapter 20](docs/chapter_20.md))
- The string "Hello, world!" is passed to the macro, and the string is then printed to the screen when we ran the binary.

### Hello Cargo

Cargo is Rust´s build system and package manager. It is used to manage Rust projects because it can handle a lot of tasks for you, such as building your code, downloading the libraries your code depends on a.k.a dependencies, and building those libraries. It comes installed with Rust. 

Check if Cargo is installed. Run this command.
```bash
cargo --version
```

If you see a version number, you have it! If you see an error, such as command not found, look at the [documentation](docs/installation.md) for your method of installation to determine how to install Cargo separately.

#### Using cargo for creating a project.

Run in the folder you want your project to be:
```bash
cargo new hello_cargo # Creates a new directory and project called hello_cargo
cd hello_cargo
```

This folder is already available. Go to the [folder](hello_cargo/).
You will see two files generated along with `.gitignore` if you specified a version control system (`--vcs=git`).

The two files would be `src/main.rs` and `Cargo.toml`. 
Open [Cargo.toml](hello_cargo/Cargo.toml).

This file is in the [TOML](https://toml.io/) format.
THe first file, `[package]`, is a section heading that indicates that the following statements are configuring a package.

The next three lines set the configuration information Cargo needs to compile your program: the name, the version, and the edition of Rust to use

The last line, `[dependencies]`, is the start of a section for you to list any of your project’s dependencies. In Rust, packages of code are referred to as crates. 

Cargo expects your source files to live inside the `src` directory. The top-level project directory is just for the README files, license information, configuration files, and anything else not related to your code. 

#### Building and Running a Cargo Project.

- Build your project by entering the following command. 
```bash
cargo build
```

The command creates the binary executable in `target/debug/hello_cargo` rather than in your current directory. 

Now this happens because the default build is a debug build. When you are ready to release your build, you can use `cargo build --release` to compile it with optimizations. This will create the binary in `target/release/hello_cargo` You can run the binary with this command. 

```bash
./target/debug/hello_cargo # or 

cargo run
```

Running `cargo build` for the first time also causes Cargo to create a new file at the top level: `Cargo.lock`. This file keeps track of the exact versions of dependencies in your project. Cargo manages the contents of the lock file with you.

Using `cargo run` is more convenient than having to remember to run `cargo build` and then use the whole path to the binary.

The command `cargo check` quickly checks your code to make sure it compiles but does not produce a binary executable.

You can check the [documentation](https://doc.rust-lang.org/cargo/) of Cargo for more info.
