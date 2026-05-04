# flashrs

`flashrs` is a small terminal flashcard viewer written in Rust. It loads a YAML deck, displays one card at a time in a TUI, and lets you flip, move through, or randomly jump between cards from the keyboard.

## Features

- Simple `yml` flashcard decks
- Front/back card display with optional hints
- Keyboard navigation in a terminal UI

## Quick Start

I've not released this on any package manager nor have I created an install script yet, so for the now just install manually:

1. Clone the tool, `git clone https://github.com/mcmanussliam/flashrs.git`
2. Navigate to the cloned repository
3. Install the tool, `cargo install --path .`
4. That's it 🎉

Now you can run specific files, I've got a few examples in the repository already:

```sh
flashrs examples/example.yml
```

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

