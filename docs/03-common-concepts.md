# 3. Common Programming Concepts

Variables, types, functions, and control flow. Comments use `//` for a line and `/* ... */` for a block.

## Variables and mutability

```rust
let mut x = 10;
x = 5; // ok because of mut
```

**Constants** are always immutable, need a type, and use `SCREAMING_SNAKE_CASE`:

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

**Shadowing** creates a new variable with the same name. You can change the type. `mut` cannot.

```rust
let x = 5;
let x = x + 1;
{
    let x = x * 2; // inner scope
    println!("{x}"); // 12
}
println!("{x}"); // 6
```

Crate: [`variables/`](../examples/03-common-concepts/variables/).

## Data types

Rust is statically typed. It will infer most types; annotate when it cannot (for example `parse`).

### Integers

| Length | Signed | Unsigned |
|--------|--------|----------|
| 8-bit  | `i8`   | `u8`     |
| 16-bit | `i16`  | `u16`    |
| 32-bit | `i32`  | `u32`    |
| 64-bit | `i64`  | `u64`    |
| 128-bit| `i128` | `u128`   |
| arch   | `isize`| `usize`  |

Signed range is `-(2^(n-1))` through `2^(n-1) - 1`.

Literals: `98_222` (decimal), `0xff` (hex), `0o77` (octal), `0b1111_0000` (binary), `b'A'` (byte, `u8` only).

Debug builds panic on overflow. `--release` wraps (two's complement). Do not rely on wrapping; it is considered a bug.

### Floats, bool, char

- Floats: `f32`, `f64` (default).
- `bool`: `true` / `false`.
- `char`: Unicode scalar, 4 bytes. `'z'`, `'ℤ'`, `'😻'`.

### Compound types

**Tuple** — fixed length, mixed types. Destructure or index with `.0`, `.1`, …

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
let (x, y, z) = tup;
let five_hundred = tup.0;
```

The empty tuple `()` is the **unit** type: “no value”. Functions that return nothing return `()`.

**Array** — fixed length, same type, stack-allocated. Use when the length is known. Vectors are for a growable list (Chapter 8).

```rust
let a = [1, 2, 3, 4, 5];
let a: [i32; 5] = [1, 2, 3, 4, 5];
let a = [3; 5]; // [3, 3, 3, 3, 3]
```

Out-of-bounds index panics at runtime.

## Functions

Snake_case names. Parameters need types. The return type comes after `->`.

```rust
fn sum(x: i32, y: i32) -> i32 {
    x + y // no semicolon: this is an expression, it returns
}
```

**Statements** do something and return no value. **Expressions** evaluate to a value.

- `let y = 6;` is a statement.
- `{ let x = 3; x + 1 }` is a block expression; it returns `4`.
- A semicolon turns an expression into a statement (`x + y;` returns `()`).
- `println!` is a macro that returns `()`.

Crate: [`functions/`](../examples/03-common-concepts/functions/).

## Control flow

No parentheses around conditions. Arms go in `{}`.

```rust
if number < 5 {
    println!("condition was true");
} else {
    println!("condition was false");
}
```

`if` is an expression. Both branches must return the same type:

```rust
let number = if condition { 5 } else { 6 };
// let number = if condition { 5 } else { "six" }; // error
```

Three loops: `loop`, `while`, `for`.

`loop` is infinite until `break`. It can return a value:

```rust
let result = loop {
    if counter == 0 {
        break counter;
    }
    counter -= 1;
};
```

Loop labels disambiguate nested loops:

```rust
'counting_up: loop {
    loop {
        if count == 2 {
            break 'counting_up;
        }
    }
}
```

`for` is the usual way to walk a collection. The compiler skips the extra bounds checks a manual `while` index loop needs:

```rust
for element in arr {
    println!("{element}");
}

for number in (1..=4).rev() {
    println!("{number}");
}
```

`1..4` is `1, 2, 3`. `1..=4` includes `4`.

Crate: [`control_flow/`](../examples/03-common-concepts/control_flow/).

## Exercises (from The Book)

| Crate | Exercise |
|-------|----------|
| [`temperature/`](../examples/03-common-concepts/temperature/) | Convert Fahrenheit ↔ Celsius |
| [`fibonacci/`](../examples/03-common-concepts/fibonacci/) | Generate Fibonacci numbers |
| [`twelve_days/`](../examples/03-common-concepts/twelve_days/) | Print *The Twelve Days of Christmas* |
