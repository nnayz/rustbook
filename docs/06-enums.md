# 6. Enums and Pattern Matching

An enum defines a type by listing its variants.

Crate: [`enums/`](../enums/).

## Define an enum

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

You can wrap an enum in a struct, or put data **on the variants themselves** — each variant can hold different types:

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```

A richer example:

```rust
enum Message {
    Quit,                       // no data
    Move { x: i32, y: i32 },    // named fields
    Write(String),              // one String
    ChangeColor(i32, i32, i32), // three i32s
}
```

That is equivalent to four separate structs, but they share one type. Enums can have `impl` blocks too:

```rust
impl Message {
    fn call(&self) {}
}
```

## `Option<T>`

`Option` encodes “something or nothing” without null:

```rust
enum Option<T> {
    None,
    Some(T),
}
```

It is in the prelude. A value of type `T` is never null. To allow absence, wrap it in `Option<T>` and handle both cases.

## `match`

`match` runs code based on which pattern hits. Arms are exhaustive — every possibility must be covered.

```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}
```

Binding `state` pulls the data out of `Quarter`.

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
```

Catch-alls: bind the leftover value with a name, or ignore it with `_`:

```rust
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),
}

match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => reroll(),
}
```

## `if let` and `let...else`

When you only care about one pattern, `if let` is shorter than a `match` with a `_ => {}` arm:

```rust
if let Some(max) = config_max {
    println!("The maximum is configured to be {max}");
}
```

`let...else` binds on match, or runs the `else` (usually `return`) if it fails:

```rust
let Coin::Quarter(state) = coin else {
    return None;
};
```
