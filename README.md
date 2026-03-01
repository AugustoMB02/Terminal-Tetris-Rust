# Terminal Tetris in Rust

A classic Tetris clone built entirely in Rust, designed to be played directly on your Linux terminal (Bash/Fish). It uses the `crossterm` library for rendering an alternate screen without cluttering your command line history.

## Features

* **Classic Grid:** 10 columns × 20 visible rows (with a 4-row hidden spawn zone).
* **7 Authentic Tetrominoes:** I, O, S, Z, T, J, L with accurate color mapping.
* **Mechanics:** 
  * Configurable descent speed (starts at 500ms).
  * Lock delays and basic wall kicks for smooth rotations near edges.
  * Soft drops and instant hard drops.
  * Full-line clears with block shifting.
* **Zero Dependencies Outside Shell:** Plays perfectly raw inside Bash or Fish.

## Installation

Ensure you have [Rust and Cargo](https://rustup.rs/) installed, then clone and run:

```bash
git clone https://github.com/YOUR_USERNAME/Terminal-Tetris-Rust.git
cd Terminal-Tetris-Rust
cargo run
```

## Controls

| Key | Action |
| --- | --- |
| `Left Arrow` | Move piece left |
| `Right Arrow` | Move piece right |
| `Down Arrow` | Soft Drop (speed up descent) |
| `Up Arrow` | Rotate piece clockwise |
| `Spacebar` | Hard Drop (instantly lock piece at bottom) |
| `Esc` / `q` / `Ctrl+C` | Quit the game |

## Built With

* [Rust](https://www.rust-lang.org/) - The programming language used.
* [crossterm](https://crates.io/crates/crossterm) - Terminal UI and event handling cross-platform library.
* [rand](https://crates.io/crates/rand) - Random number generation for piece spawning.

## License

This project is open-source and available under the MIT License.
