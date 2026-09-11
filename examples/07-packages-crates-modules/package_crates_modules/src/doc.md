/*
 * Packages: Cargo feature that lets you build, test, and share crates
 * Crates: a tree of modules that produces a library or executable
 * Modules and use: Let you control the orgsanisation, scope and privacy of paths
 * Paths: A way of naming an item, such as a struct, function, or module
 */
 
    /*
     * A crate is the smallest amount of code that the Rust compiler considers at a time.
     *
     * Can come in two forms: binary crate or library crate.
     *
     * Binary crates are programs you can compule to an executable that you can run, such as command line program or a server.
     *
     * Library crates do not have a main function and they do not compile to an executable. Instead, they define functionality intended to be shared with multiple projects.
     *
     *
     * Crate root is a source file that the Rust compiler starts from and makes up the root module of your crate.
     *
     *
     * A package is a bundle of one or more crates that provide a set of functionality. A package contains a Cargo.toml file that describes how to build those crates.
     *
     * Cargo is actually a package that contains the binary crate for the CLI tool.
     *
     * A package can contain as many binary crates as one likes, but at most only one library crate.
     *
     * A package must contain at least one crate, whether library or binary
     *
     * If a package contains src/main.rs and src/lib.rs, it has two crates: a binary and a library, both with the same name as the package. A package can have multiple binary crates by placing files in the src/bin directory: Each file will be a separate binary crate.
     */
