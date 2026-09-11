# 1. Getting Started

Rust source files use the `.rs` extension. Multi-word names are snake_case: `hello_world.rs`.

## Hello, world

```rust
fn main() {
    println!("Hello, world!");
}
```

Compile with `rustc`, then run the binary:

```bash
rustc main.rs
./main
```

This repo has a root `main.rs` that does exactly that.

### Anatomy

- `fn main()` is the entry point of a binary crate. The body must be wrapped in `{}`.
- `println!` is a **macro**, not a function. The `!` is the giveaway. Macros generate code; more on them in [Chapter 20](20-advanced-features.md) (not written yet).
- The string `"Hello, world!"` is passed to the macro and printed when the binary runs.

## Hello Cargo

Cargo is Rust's build system and package manager. It builds your code, downloads dependencies (crates), and builds those too. It ships with Rust.

```bash
cargo --version
```

If that fails, install the toolchain from [rustup.rs](https://rustup.rs/). See also [installation notes](installation.md) (not written yet).

### Create a project

```bash
cargo new hello_cargo
cd hello_cargo
```

This folder already exists: [`hello_cargo/`](../hello_cargo/).

Cargo generates `src/main.rs`, `Cargo.toml`, and a `.gitignore` if you asked for git (`--vcs=git`).

`Cargo.toml` is [TOML](https://toml.io/). `[package]` is the package config (name, version, edition). `[dependencies]` is where crates go.

Cargo expects source in `src/`. The project root is for README, license, config — not code.

### Build and run

```bash
cargo build                 # debug binary at target/debug/hello_cargo
./target/debug/hello_cargo

cargo run                   # build + run
cargo check                 # compile-check, no binary
cargo build --release       # optimized binary at target/release/hello_cargo
```

The first `cargo build` also writes `Cargo.lock`, which pins exact dependency versions.

`cargo run` is the usual loop. `cargo check` is faster when you only care that it compiles.

Full Cargo docs: [doc.rust-lang.org/cargo](https://doc.rust-lang.org/cargo/).

## Code in this repo

| Path | What it is |
|------|------------|
| [`main.rs`](../main.rs) | Hello world, compiled with `rustc` |
| [`hello_cargo/`](../hello_cargo/) | Same program as a Cargo project |
