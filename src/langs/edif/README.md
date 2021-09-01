# edifier
Like an emulsifier, edifier aims to allow you to mix rust and edif. 

The goal of this is to turn it into a crate to allow the generation of edif files programatically from a Rust program.

# Project Structure
The project has the following areas:
* examples: this is a good point of entry as it will quicky show you a template for your application.
* src: this is the place where we have stored the code, separated in crates.
    * edifier: This crate is the core of the project, provides generic edif utilities.
    * platforms: This crate hosts a non-comprehensive list of platform specific support functions, mainly for unit-testing/example.
* others: random files to support development.

# Still under construction
Please do not integrate just yet as it is still early days...
