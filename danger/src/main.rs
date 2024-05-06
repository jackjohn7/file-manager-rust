mod commands;
mod events;
mod state;
mod ui;

use clap::Parser;
use commands::args;
use std::{
    env::{current_dir, set_current_dir},
    io::{self, stdout, Write},
};
use utils::files_in_dir;

use crate::{
    events::{key_events::handle_events, triggers::handle_triggers},
    state::{AppConfig, AppMode, AppState},
    ui::render_ui::ui,
};
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::*;

fn main() -> io::Result<()> {
    let args = args::DangerArgs::parse();

    set_current_dir(args.input).expect("Invalid input provided. Ensure path is valid");

    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut app_state = AppState {
        mode: AppMode::Browse,
        line: 0,
        scroll_offset: 0,
        files: Vec::new(),
        trigger: None,
        pg_height: None,
        search_string: String::new(),
        command_string: String::new(),
        config: AppConfig::default(),
    };
    app_state.files = files_in_dir(current_dir().unwrap().as_path());
    app_state.config.show_full_path = args.full_paths;

    let mut should_quit = false;
    while !should_quit {
        // if trigger update, need to fetch files again
        let _ = handle_triggers(&mut app_state);
        terminal.draw(|frame| ui(frame, &mut app_state))?;
        should_quit = handle_events(&mut app_state)?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    if args.source {
        stdout()
            .write(
                format!(
                    "cd {}",
                    current_dir().unwrap().as_os_str().to_str().unwrap()
                )
                .as_bytes(),
            )
            .unwrap();
    }

    Ok(())
}
