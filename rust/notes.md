# Chapter 7

* Packages: A Cargo feature that lets you build, test, and share crates
* Crates: A tree of modules that produces a library or executable
* Modules and use: Let you control the organization, scope, and privacy of paths
* Paths A way of naming an item, such as a struct, function, or module

A *crate* is a binary or library.

*Crate root* is a source file that the Rust compiler starts from and makes up the root module of your crate.

A *package* is one or more crates that provide a set of functionality.

A *package* contains a Cargo.toml file that describes how to build those crates.

A package must contain zero or one *library crates*.

A package can contain as many *binary crates* as you'd like.

A package should *contain at least one* crate(library or binary).

Cargo follows a convention that src/main.rs is the crate root of a *binary crate* with the same name as the package.

If the package directory contains src/lib.rs, Cargo knows that the package contains a *library crate* with the same name as the package, and src/lib.rs is its crate root.

Cargo passes the crate root files to rustc to build the library or binary.

A package can have multiple *binary crates* by placing files in the src/bin directory: each file will be a separate binary crate.

*Modules* let us organize code within a crate into groups for readability and easy reuse. Modules control the privacy of items.

*use* keyword allows us to bring a path into a scope once and then call the items in that path as if they're local items.

# Chapter 9

When the *panic!* macro executes, your program will print a failure message, unwind and clean up the stack, and then quit.
