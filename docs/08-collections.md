# 8. Common Collections

Heap-growing data structures from `std`.

- **Vector** `Vec<T>`: list of `T`, next to each other
- **String**: owned UTF-8 text
- **Hash map** `HashMap<K, V>`: keys to values

Crate: [`collections/`](../examples/08-collections/collections/).

## Vectors

```rust
let v: Vec<i32> = Vec::new();
let v = vec![1, 2, 3];

let mut v = Vec::new();
v.push(5);
```

Read with indexing (`&v[2]`, panics if missing) or `.get(2)` (`Option<&T>`).

The borrow checker will not let you hold a reference into a vector and then `push` — `push` may reallocate.

```rust
for j in &v { /* immutable */ }
for j in &mut v {
    *j += 10;
}
```

A vector's element type is one type. Mix shapes with an enum:

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
```

The vector is dropped when it goes out of scope, and so are its elements.

## Strings

`String` and `&str` are UTF-8. Literals are `&str` stored in the binary.

Create: `String::new()`, `String::from("...")`, `"...".to_string()` (`to_string` works on anything that implements `Display`).

Grow:

- `push_str(&str)` appends a slice, does not take ownership
- `push(char)` appends one character
- `s1 + &s2` takes ownership of `s1` (`add(self, s: &str) -> String`)
- `format!("{s1}-{s2}-{s3}")` does not take ownership

You cannot index a `String` with `s[0]`. Internally it is a `Vec<u8>`. A Unicode character may be more than one byte, and it is unclear whether you wanted a byte, a `char`, a grapheme, or a slice.

Slicing `&hello[0..4]` is allowed but panics if you split a character. Iterate with `.chars()` or `.bytes()`.

## Hash maps

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

let score = scores.get(&team_name).copied().unwrap_or(0);

for (key, value) in &scores {
    println!("{key}: {value}");
}
```

Data lives on the heap. All keys share a type; all values share a type.

`Copy` types are copied in. Owned types like `String` are **moved** in; the map becomes the owner.

```rust
mp.insert(k, v);                      // overwrite
mp.entry(k).or_insert(50);            // insert only if missing
let count = mp.entry(word).or_insert(0);
*count += 1;                          // update from the old value
```

Default hasher is SipHash (DoS-resistant, not the fastest). Swap it by providing a type that implements `BuildHasher`.

## Exercises (from The Book)

Package [`exercises/`](../examples/08-collections/exercises/) — three binaries in `src/bin/`:

| Binary | Run | Exercise |
|--------|-----|----------|
| `mediam_mode` | `cargo run -p exercises --bin mediam_mode` | Median and mode of a list of integers |
| `pig_latin` | `cargo run -p exercises --bin pig_latin` | Convert a word to pig latin (UTF-8-aware enough to use `.chars()`) |
| `names_in_company` | `cargo run -p exercises --bin names_in_company` | Add people to departments; list by department, sorted |
