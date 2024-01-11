use std::{io, env::set_current_dir};

use crate::state::{AppMode, AppState};
use crate::events::triggers::AppTrigger;
use crossterm::event::{self, Event, KeyCode};
use utils::FolderItem;


pub fn handle_events(state: &mut AppState) -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }
            else {
                match state.mode {
                    // WHEN IN BROWSE MODE
                    AppMode::Browse => {
                        if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('j') {
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
                        else if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('k') {
                            if state.line > 0 {
                                state.line -= 1;
                            }
                            // when moving 5 items from the bottom, scroll up a bit
                            if (state.scroll_offset > 0) && (state.files.len() as i32 - state.line as i32) > 5{
                                state.scroll_offset -= 1;
                            }
                        }
                        else if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('l') {
                            if let FolderItem::Directory(dir) = &state.files[state.line] {
                                set_current_dir(&dir.path).unwrap();
                                state.line = 0;
                                state.trigger = Some(AppTrigger::Refresh);
                            }
                        }
                        else if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('h') {
                            set_current_dir("..").unwrap();
                            state.line = 0;
                            state.trigger = Some(AppTrigger::Refresh);
                        }
                        else if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('/') {
                            // put the manager into search mode
                            state.mode = AppMode::BrowseSearch;
                        }
                    }
                    // WHEN IN BROWSESEARCH MODE
                    AppMode::BrowseSearch => {
                        if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Esc {
                            // put the manager into search mode
                            state.mode = AppMode::Browse;
                        }

                    }
                }
            }
        }
    }
    Ok(false)
}
