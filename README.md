# playmill

Playmill implements a simple factory-game simulation in Rust

Author: Jared Rhine @ jared@wordzoo.com

## Background

The author is not an experienced Rust developer. This repository was
built is a fun learning exercise, inspired by the "factory simulation"
genre of games (Satisfactory and Dyson Sphere Project primarily, based
on Factorio). Do not take this code very seriously.

## Status

Works:

* Compiles in Rust
* Displays the status of the world using ASCII with a terminal
* Runs in a continuous loop at 4 frames per second

Future directions:

* Buildings can be connected
* Conveyors pass their input to their output
* A new building that can create resources
* Multiple kinds of resources
* More idiomatic Rust code
* Split into multiple files
* Tests
* A new building that can combine two kinds of resources to make a new kind

## Using Playmill

1. Get this repository:

```shell
git clone https://github.com/jaredrhine/playmill
cd playmill
```

1. Install `asdf` or otherwise build a modern Rust installation
   providing `cargo`.

```shell
asdf install
```

1. Use `cargo` to build and execute the playmill application.

```shell
cargo run
```

1. Playmill will clear the screen and print a view of the simulated
   world at 4 frames per second.

   * There will likely be flashing as your terminal repaints frequently.

   * There is an on-screen reminder of two available keys.

     * **Press "q" to quit.**
     * Press the spacebar to pause and to resume.

   * The terminal support will likely only work on a Unix-like terminal.
     Windows-based shells may have problems.
     
## Screenshot

<img width="465" alt="image" src="https://user-images.githubusercontent.com/81832/152639102-9eb78927-a000-4433-b509-eea43ceb0a69.png">

