use std::{io::{self, stdout}, env::{current_dir, set_current_dir}, cmp::Ordering};
use utils::{files_in_dir, FolderItem};
use std::fmt::Write;

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};

enum AppMode {
    Browse,
}

enum AppTrigger {
    Refresh
}

struct AppState {
    mode: AppMode,
    line: usize,
    scroll_offset: usize,
    files: Vec<FolderItem>,
    trigger: Option<AppTrigger>,
    pg_height: Option<usize>,
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut app_state = AppState{ mode: AppMode::Browse, line: 0, scroll_offset: 0, files: Vec::new(), trigger: None, pg_height: None };
    app_state.files = files_in_dir(current_dir().unwrap().as_path());

    let mut should_quit = false;
    while !should_quit {
        // if trigger update, need to fetch files again
        if let Some(trigger) = app_state.trigger {
            match trigger {
                AppTrigger::Refresh => {
                    app_state.files = files_in_dir(current_dir().unwrap().as_path());
                    app_state.scroll_offset = 0;
                }
            }
            app_state.trigger = None;
        }
        terminal.draw(|frame| ui(frame, &mut app_state))?;
        should_quit = handle_events(&mut app_state)?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn handle_events(state: &mut AppState) -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }
            else if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('j') {
                match state.mode {
                    AppMode::Browse => {
                        if (state.line as i32) < (state.files.len() as i32)-1 {
                            state.line += 1;
                        }
                        // when moving 5 items from the bottom, scroll down a bit
                        if let Some(pg_height) = state.pg_height {
                            if ((pg_height as i32 - state.line as i32) < 5) && (state.files.len() as i32 - state.line as i32) > 5 {
                                state.scroll_offset += 1;
                            }
                        }
                    }
                }
            }
            else if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('k') {
                match state.mode {
                    AppMode::Browse => {
                        if state.line > 0 {
                            state.line -= 1;
                        }
                        // when moving 5 items from the bottom, scroll up a bit
                        if (state.scroll_offset > 0) && (state.files.len() as i32 - state.line as i32) > 5{
                            state.scroll_offset -= 1;
                        }
                    }
                }
            }
            else if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('l') {
                match &state.files[state.line] {
                    FolderItem::Directory(dir) => {
                        set_current_dir(&dir.path).unwrap();
                        state.line = 0;
                        state.trigger = Some(AppTrigger::Refresh);
                    },
                    _ => {}
                }
            }
            else if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('h') {
                set_current_dir("..").unwrap();
                state.line = 0;
                state.trigger = Some(AppTrigger::Refresh);
            }
        }
    }
    Ok(false)
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
        .title(format!("{}", current_dir().unwrap().to_str().unwrap()))
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
