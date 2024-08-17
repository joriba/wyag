# WYAG - Rust

A simple implementation of [the Git version control system](https://git-scm.com/).

WYAG - Rust has been created as a learning experience following the 
[Write yourself a git](https://wyag.thb.lt/) python tutorial, but adapted to rust.

## Usage
After cloning, run `cargo run -- init` to create a wyag repo.

Currently, wyag - rust supports the following wyag commands (equivalent, though often less powerful,
to their respective git commands):
* `wyag init [path]`: create a new repository at the given path. Default is the current directory.
