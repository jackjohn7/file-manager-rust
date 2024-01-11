mod events;
mod state;

use std::{io::{self, stdout}, env::current_dir, cmp::Ordering};
use utils::{files_in_dir, FolderItem};
use std::fmt::Write;

use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};
use crate::{
    state::{
        AppMode,
        AppState,
    },
    events::{
        key_events::handle_events,
        triggers::handle_triggers
    }
};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut app_state = AppState{ mode: AppMode::Browse, line: 0, scroll_offset: 0, files: Vec::new(), trigger: None, pg_height: None };
    app_state.files = files_in_dir(current_dir().unwrap().as_path());

    let mut should_quit = false;
    while !should_quit {
        // if trigger update, need to fetch files again
        let _ = handle_triggers(&mut app_state);
        terminal.draw(|frame| ui(frame, &mut app_state))?;
        should_quit = handle_events(&mut app_state)?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}


fn list_files(files: &Vec<FolderItem>) -> String {
    let mut result = String::new();
    for f in files {
        writeln!(result, "{}", match f {
            FolderItem::Directory(dir) => format!("Dir        {}", dir.path.to_str().unwrap()),
            FolderItem::File(fil) => format!("File       {}", fil.path.to_str().unwrap()),
        },).unwrap();
    }
    result
}

fn ui(frame: &mut Frame, state: &mut AppState) {
    let files = list_files(&state.files);

    let paragraphs: Vec<Paragraph> = files
        .lines()
        .enumerate()
        //.filter(|(line_idx, _)| line_idx >= &state.scroll_offset)
        .map(|(line_idx, line_txt)| {
            if line_idx == state.line {
                Paragraph::new(line_txt)
                    .style(Style::new().white().on_blue().bold())
            } else {
                Paragraph::new(line_txt)
            }
        })
        .collect();

    let block = Block::default()
        .title(format!("{} Location: {}", match state.mode {
            AppMode::Browse => "Browsing",
            AppMode::BrowseSearch => "Searching"
        }, current_dir().unwrap().to_str().unwrap()))
        .borders(Borders::ALL);

    frame.render_widget(block.clone(), frame.size());

    let block_area = block.inner(frame.size());
    let paragraph_height = block_area.height.saturating_sub(2); // Adjust for borders
    state.pg_height = Some(paragraph_height as usize);

    // paragraph_height as usize +state.scroll_offset
    for (i, paragraph) in paragraphs[state.scroll_offset..(match state.files.len().cmp(&state.pg_height.unwrap()) {
        Ordering::Greater => state.pg_height.unwrap() + state.scroll_offset,
        _ => state.files.len()
    })].iter().take(paragraph_height as usize).enumerate() {
        frame.render_widget(
            paragraph.clone(),
            Rect::new(block_area.x, block_area.y + i as u16, block_area.width, 1),
        );
    }
}
