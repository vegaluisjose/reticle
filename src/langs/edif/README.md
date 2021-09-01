# edif
The goal of this is to turn it into a crate to allow the generation of edif files programatically from a Rust program.

# Crate Structure
This crate has the following areas:
* examples: this is a good point of entry as it will quicky show you a template for your application.
* src: this is the place where we have stored the code, separated in crates.
    * edifier: This crate is the core of the project, provides generic edif utilities.
    * platforms: This crate hosts a non-comprehensive list of platform specific support functions, mainly for unit-testing/example.
