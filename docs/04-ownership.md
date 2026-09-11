# 4. Understanding Ownership

Ownership is the set of rules that govern how a Rust program manages memory.

## Stack and heap

Both are memory available at runtime.

- **Stack**: LIFO. Data must have a known, fixed size. Pushing is cheap — the next slot is always the top.
- **Heap**: less organized. You ask the allocator for a chunk; it finds space and returns a pointer. That pointer (fixed size) lives on the stack. Following it to the data is slower.

Calling a function pushes its arguments and locals onto the stack. Returning pops them.

Ownership exists to:

1. Track who is using heap data
2. Avoid duplicating heap data
3. Free unused heap data so you do not run out of memory

## The rules

1. Each value in Rust has an owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value is dropped.

Crate: [`ownership/`](../examples/04-ownership/ownership/).

## `String` vs string literals

String literals (`"hello"`) are baked into the binary and immutable.

`String` is growable, heap-allocated:

```rust
let mut s = String::from("Hello");
s.push_str(", world!");
```

When `s` goes out of scope, Rust calls `drop` and returns the heap memory.

## Move, clone, copy

Integers are `Copy` — assigning copies the bits on the stack:

```rust
let x = 5;
let y = x; // both valid
```

`String` is a pointer + length + capacity on the stack, bytes on the heap. `let s2 = s1;` copies the stack header, **not** the heap. To avoid a double free, `s1` is invalidated. That is a **move**, not a shallow copy.

```rust
let s1 = String::from("Hello");
let s2 = s1;
// println!("{s1}"); // error: s1 moved
```

Deep copy the heap with `.clone()`:

```rust
let s2 = s1.clone();
```

You cannot implement `Copy` on a type that implements `Drop`.

Reassigning a variable drops the old value immediately:

```rust
let mut s = String::from("Hello!");
s = String::from("ahoy"); // "Hello!" is freed here
```

Passing a value into a function moves or copies it the same way assignment does. Returning a value transfers ownership back.

```rust
fn takes_ownership(some_string: String) { /* ... */ } // dropped at end
fn makes_copy(some_integer: i32) { /* ... */ }        // i32 is Copy
```

Returning a `(String, usize)` tuple is a way to give the string back along with extra data. References (next section) are the usual fix.

## References and borrowing

A reference is an address you can follow. It does not own the data. Creating one is **borrowing**.

Crate: [`references_borrowing/`](../examples/04-ownership/references_borrowing/).

```rust
fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope; the String is not dropped
```

`&s1` does not move `s1`. The opposite of `&` is dereference, `*`.

Immutable borrows cannot mutate. Use `&mut`:

```rust
fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}
```

Rules that prevent data races:

- Either one mutable reference, **or** any number of immutable references
- References must always be valid (no dangling pointers)

This is a data race: two or more pointers to the same data, at least one writing, no synchronization.

```rust
let mut s = String::from("hello");
let r1 = &mut s;
let r2 = &mut s; // error
```

Sequential mutable borrows are fine once the first one has ended:

```rust
{
    let r4 = &mut s;
    println!("{r4}");
} // r4 ends
let r3 = &mut s; // ok
```

A function that returns a reference to its own local will not compile — that local is dropped, so the reference would dangle.

## Slices

A slice is a reference to a contiguous sequence. It does not own the data.

Crate: [`slices/`](../examples/04-ownership/slices/).

```rust
let s = String::from("hello world");
let hello = &s[0..5];
let world = &s[6..11];
```

Range sugar: `&s[..2]` from the start, `&s[3..]` to the end, `&s[..]` the whole thing.

`first_word` that returns an index goes stale if the string is cleared. Returning a slice ties the result to the data:

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
```

Prefer `&str` over `&String` so the function takes both `String` slices and string literals. Literals **are** `&str`.

Array slices work the same way:

```rust
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3]; // type &[i32], equals &[2, 3]
```
