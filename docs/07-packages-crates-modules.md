# 7. Packages, Crates, and Modules

How a project is cut into pieces as it grows.

- **Package**: a Cargo bundle. One `Cargo.toml`. At least one crate. At most one library crate. Any number of binary crates.
- **Crate**: the unit the compiler considers. Binary (has `main`, becomes an executable) or library (shared code, no `main`).
- **Crate root**: the file the compiler starts from (`src/main.rs` or `src/lib.rs`). That file is the crate's root module.
- **Module** + `use`: organization, scope, and privacy.
- **Path**: how you name an item (struct, function, module).

If a package has both `src/main.rs` and `src/lib.rs`, it has two crates with the same package name. Extra binaries go in `src/bin/`.

Notes also live in [`package_crates_modules/src/doc.md`](../package_crates_modules/src/doc.md). Crates: [`restaurant/`](../restaurant/) (library), [`backyard/`](../backyard/) (modules in files).

## Module tree

Define the tree in the crate root (`lib.rs` for a library):

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
```

Items are private by default. `pub` exposes them to the parent. A parent cannot see a child's private items; a child can use `super::` to reach the parent.

## Paths

- **Absolute**: starts at the crate root (`crate::front_of_house::hosting::add_to_waitlist()`).
- **Relative**: starts at the current module (`front_of_house::hosting::add_to_waitlist()`, or `self`, `super`, an identifier).

```rust
mod back_of_house {
    pub fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
    fn cook_order() {}
}
```

## Public structs and enums

Making a struct `pub` does not make its fields public. Name each field `pub` if callers should write it. Enums are the opposite: `pub enum` makes every variant public.

```rust
pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String, // private: only construct via a method
}

pub enum Appetizer {
    Soup,
    Salad,
}
```

## `use`

Bring a path into scope. Idiom: bring the parent module for functions (`use crate::front_of_house::hosting;` then `hosting::add_to_waitlist()`), bring the type itself for structs and enums.

```rust
use back_of_house::Breakfast;
```

Nested paths and glob:

```rust
use std::{cmp::Ordering, io};
use std::io::{self, Write};
use std::collections::*; // glob
```

`as` renames: `use std::fmt::Result as FmtResult;`.

## Modules in files

`mod garden;` in the crate root looks for `src/garden.rs` or `src/garden/mod.rs`. Submodules of `garden` go in `src/garden/`:

```
backyard/src/
  main.rs          // crate::garden
  garden.rs        // pub mod vegetables;
  garden/
    vegetables.rs  // pub struct Asparagus
```

See [`backyard/src/main.rs`](../backyard/src/main.rs).
