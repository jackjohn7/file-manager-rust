# File Managers in Rust


I'm writing some basic file management tools in Rust to get a feel for
some of the filesystem functions.

The plan is to implement the following:
- GUI with GTK (planned)
- GUI with Tauri (planned)
- Danger (TUI with Ratatui (started))

I will be doing the same thing in Go!

# Danger (TUI)

<img src="/assets/browsing_mode.png">

Danger is a rust-based TUI file manager inspired by Ranger as you can probably tell.

## TODO

### Interface

- Allow command to take a folder as argument ("." is default)
- `--nav` flag navigates user to their last visited folder upon exiting
- `--full-paths` flag specifies that files and directories should display full path rather than just the name

### Actions

- <kbd>a</kbd> allows a user to create a new file
  - <kbd>ESC</kbd> cancels this
  - <kbd>ENTER</kbd> confirms this
- <kbd>y</kbd> initiates a yank (copy)
  - <kbd>y</kbd> completes a yank
  - <kbd>ESC</kbd> cancels a yank
  - *Note*: In the case of name collision, a "(n)" is appended where $n$ is a counting integer.
- <kbd>d</kbd> allows a user to delete a file or folder recursively
  - <kbd>d</kbd> allows a user to perform a cut
  - <kbd>ENTER</kbd> confirms
  - <kbd>ESC</kbd> cancels this
- <kbd>l</kbd> on a file opens the file in default application
- <kbd>p</kbd> Allows a user to paste copied or cut content

### Configuration

- `danger.toml` in "home" directory, ".config" directory, or installation location can configure Danger.
  - *check order:* `home` -> `.config` -> `<install_location>`
- Example `danger.toml` in installation directory
- Allow user to set color theme in `danger.toml`

## Usage

`git clone https://github.com/jackjohn7/file-manager-rust`

`cd file-manager-rust`

`cargo run --bin tui`

## Controls

- Use `h`, `j`, `k`, and `l` to navigate through directories
- Use `b` and `t` to scroll to bottom and top respectively
- Use `/` to begin searching
  - `ESC` will exit search mode and reset search string
    - *Note*: This is also how you remove your filter afterwards. `/-ESC`
  - `ENTER` will exit search mode and apply search string
- Use `n` to toggle the numbering of files
- Use `f` to toggle display of full path for files

