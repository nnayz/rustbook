# 5. Using Structs to Structure Related Data

A struct packages named fields into one type.

Crates: [`structs/`](../examples/05-structs/structs/), [`methods/`](../examples/05-structs/methods/).

## Define and instantiate

```rust
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

let user = User {
    active: true,
    username: String::from("username"),
    email: String::from("user@example.com"),
    sign_in_count: 1,
};
```

Field init shorthand: if the variable name matches the field, write `username` instead of `username: username`.

Struct update syntax moves remaining fields from another instance:

```rust
let user3 = User {
    email: String::from("another@example.com"),
    ..user
};
```

`user` can no longer be used if a moved field (like `String`) was taken. Fields that implement `Copy` (`bool`, `u64`) are copied.

`#[derive(Debug)]` lets you print with `{:?}` or pretty-print with `{:#?}`. `dbg!` takes an expression, prints it (file + line), and returns ownership.

## Tuple structs and unit-like structs

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
let Point(x, y, z) = origin;

#[derive(Debug)]
struct AlwaysEqual; // no fields — useful as a marker that can hold traits
```

`Color` and `Point` are different types even though their fields look the same.

## Ownership of struct data

These examples store owned `String`, not `&str`. Each instance owns its data for as long as the struct lives. Borrowed fields need lifetimes (Chapter 10).

## Refactor toward a struct

A rectangle area starts as two parameters, then a tuple, then a struct so the fields have names and the value can be reused:

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
```

Borrow the struct so `main` keeps it.

## Methods

Methods are functions in an `impl` block. The first parameter is `self` (the instance).

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

rect.area();
let sq = Rectangle::square(20);
```

`&self` is short for `self: &Self`. Take `&mut self` to mutate, `self` to take ownership.

A method can share a name with a field (`fn width(&self) -> bool`). Call it with `()`. The field is `rect.width`.

**Associated functions** do not take `self`. They are namespaced on the type (`Rectangle::square`). Multiple `impl` blocks on the same type are allowed.
