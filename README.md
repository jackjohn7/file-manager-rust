# File Managers in Rust

<img src="/assets/browsing_mode.png">

I'm writing some basic file management tools in Rust to get a feel for
some of the filesystem functions.

The plan is to implement the following:
- GUI with GTK (planned)
- GUI with Tauri (planned)
- TUI with Ratatui (started)

I will be doing the same thing in Go!

# TUI Usage

`git clone https://github.com/jackjohn7/file-manager-rust`

`cd file-manager-rust`

`cargo run --bin tui`

## Controls

- Use `h`, `j`, `k`, and `l` to navigate through directories
- Use `b` and `t` to scroll to bottom and top respectively
- Use `/` to begin searching
  - `ESC` will exit search mode and reset search string
  - `ENTER` will exit search mode and apply search string
- Use `n` to toggle the numbering of files

