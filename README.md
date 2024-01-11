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

Danger is a minimal rust-based TUI file manager inspired by Ranger and Vim.

Danger is a work-in-progress, and I wouldn't recommend anyone use it as their
primary file manager. There is a chance a bug results in you deleting something 
you didn't intend to. You have been warned.

## TODO

### Interface

- `--paths-relative` flag specifies that paths should be displayed relative to where 
danger was opened.

### Actions

Some of these require the addition of support for multi-button actions. Action composition.

- Number keys allow a user to specify the yanking or cutting of multiple files/folders

- <kbd>a</kbd> allows a user to create a new file
  - <kbd>ESC</kbd> cancels this
  - <kbd>ENTER</kbd> confirms this
- <kbd>y</kbd> initiates a yank (copy)
  - <kbd>y</kbd> completes a yank
  - <kbd>ESC</kbd> cancels a yank
  - *Note*: In the case of name collision, a "(n)" is appended where $n$ is a counting integer.
- <kbd>:</kbd> followed by a number and <kbd>ENTER</kbd> will jump to a line
- <kbd>{</kbd> and <kbd>}</kbd> will jump down and up a page length respectively
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

## Args

- `--input` flag sets the current directory
- `--full-paths` flag specifies that files and directories should display full path rather than just the name

## Controls

- Use <kbd>h</kbd>, <kbd>j</kbd>, <kbd>k</kbd>, and <kbd>l</kbd> to navigate through directories
- Use <kbd>b</kbd> and <kbd>t</kbd> to scroll to bottom and top respectively
- Use <kbd>/</kbd> to begin searching
  - <kbd>ESC</kbd> will exit search mode and reset search string
    - *Note*: This is also how you remove your filter afterwards. <kbd>/-ESC</kbd>
  - <kbd>ENTER</kbd> will exit search mode and apply search string
- Use <kbd>n</kbd> to toggle the numbering of files
- Use <kbd>f</kbd> to toggle display of full path for files

