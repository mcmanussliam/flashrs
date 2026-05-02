# flashrs

`flashrs` is a small terminal flashcard viewer written in Rust. It loads a YAML deck, displays one card at a time in a TUI, and lets you flip, move through, or randomly jump between cards from the keyboard.

## Features

- YAML-backed flashcard decks
- Front/back card display with optional hints
- Keyboard navigation in a terminal UI
- Sequential previous/next navigation
- Random card selection

## Requirements

- Rust 2024 edition toolchain
- A terminal that supports alternate-screen TUI applications

## Usage

Run the example deck:

```sh
cargo run -- examples/example.yml
```

Run another deck:

```sh
cargo run -- path/to/deck.yml
```

Build a release binary:

```sh
cargo build --release
./target/release/flashrs examples/example.yml
```

## Controls

| Key | Action |
| --- | --- |
| `F` or `Enter` | Flip the current card |
| `N` or `Right` | Move to the next card |
| `P` or `Left` | Move to the previous card |
| `R` or `Space` | Jump to a random card |
| `Q` or `Esc` | Quit |

## Deck Format

Decks are YAML files with a top-level `cards` list. Each card needs a `front` and `back`; `hint` is optional.

```yaml
cards:
  - front: What command initializes a new Rust project?
    back: cargo new my_app
    hint: Think Cargo.

  - front: What macro prints to stdout with a newline?
    back: println!
    hint: Ends with an exclamation mark.
```

The example files also include metadata such as `name`, `author`, and `year`. These fields are harmless, but the app currently only reads `cards`.

## Examples

This repository includes:

- `examples/example.yml`: a small Rust-focused sample deck
- `examples/ns.yml`: Network Systems study cards
- `examples/os.yml`: Operating Systems study cards
