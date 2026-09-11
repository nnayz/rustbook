# 9. Error Handling

Rust has no exceptions. Two paths:

- **Recoverable**: something like “file not found”. Report it, retry, or propagate. Type: `Result<T, E>`.
- **Unrecoverable**: a bug, like an out-of-bounds index. Macro: `panic!`.

Crate: [`error_handling/`](../examples/09-error-handling/error_handling/). `File::open("hello.txt")` is relative to the current directory, so run it from the crate:

```bash
cd examples/09-error-handling/error_handling
cargo run
```

## `panic!`

`panic!` stops execution. By default Rust **unwinds**: walks the stack and drops data. That is work. You can abort instead (immediate stop, OS cleans up) via Cargo profile settings.

A **backtrace** is the list of functions that led to the panic. Run with `RUST_BACKTRACE=1`.

```rust
panic!("crash and burn");
let v = vec![1, 2, 3];
v[99]; // panics
```

## `Result<T, E>`

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`File::open` returns `Result<File, Error>`:

```rust
use std::{fs::File, io::ErrorKind};

let greeting_file = match File::open("hello.txt") {
    Ok(file) => file,
    Err(e) => match e.kind() {
        ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => panic!("Problem creating the file {e:?}"),
        },
        _ => panic!("Problem opening the file: {e:?}"),
    },
};
```

`expect` / `unwrap` panic on `Err`. Fine in examples and when a failure means a bug. Prefer `match` or `?` when the caller should decide.

## When to panic

Panic when the error is a bug: invalid state, broken invariant, caller violated a contract.

Return `Result` when the error is expected in normal use: bad input, missing file, network blip.

More in The Book: [To `panic!` or Not to `panic!`](https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html). Helpers like `?`, `unwrap_or_else`, and custom error types belong here too — notes not written yet beyond the crate above.
