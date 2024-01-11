use std::{cmp::Ordering, env::current_dir};

use ratatui::{prelude::*, widgets::*};
use utils::FolderItem;
use std::fmt::Write;

use crate::state::{AppState, AppMode, AppConfig};

pub fn list_files(files: &Vec<FolderItem>, config: &AppConfig) -> String {
    let mut result = String::new();
    for (idx, f) in files.into_iter().enumerate() {
        if config.numbering {
            write!(result, "{}  ", idx+1);
        }
        writeln!(result, "{}", match f {
            FolderItem::Directory(dir) => format!("Dir        {}", dir.path.to_str().unwrap()),
            FolderItem::File(fil) => format!("File       {}", fil.path.to_str().unwrap()),
        },).unwrap();
    }
    result
}

pub fn ui(frame: &mut Frame, state: &mut AppState) {
    match state.mode {
        AppMode::Browse => browse_ui(frame, state),
        AppMode::BrowseSearch => browse_search_ui(frame, state)
    }
}

fn browse_ui(frame:&mut Frame, state: &mut AppState) {
    let files = list_files(&state.files, &state.config);

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

fn browse_search_ui(frame: &mut Frame, state: &mut AppState) {
    let files = list_files(&state.files, &state.config);
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(85),
            Constraint::Percentage(15),
        ])
        .split(frame.size());

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

    let file_block = Block::default()
        .title(format!("{} Location: {}", match state.mode {
            AppMode::Browse => "Browsing",
            AppMode::BrowseSearch => "Searching"
        }, current_dir().unwrap().to_str().unwrap()))
        .borders(Borders::ALL);

    frame.render_widget(file_block.clone(), layout[0]);

    let file_block_area = file_block.inner(layout[0]);
    let paragraph_height = file_block_area.height.saturating_sub(2); // Adjust for borders
    state.pg_height = Some(paragraph_height as usize);

    // paragraph_height as usize +state.scroll_offset
    for (i, paragraph) in paragraphs[state.scroll_offset..(match state.files.len().cmp(&state.pg_height.unwrap()) {
        Ordering::Greater => state.pg_height.unwrap() + state.scroll_offset-1,
        _ => state.files.len()
    })].iter().take(paragraph_height as usize).enumerate() {
        frame.render_widget(
            paragraph.clone(),
            Rect::new(file_block_area.x, file_block_area.y + i as u16, file_block_area.width, 1),
        );
    }
    let search_block = Block::default()
        .title("Search (ESC to Cancel, ENTER to Apply and resume browse)")
        .borders(Borders::ALL);
    let search_block_area = search_block.inner(layout[1]);
    frame.render_widget(search_block.clone(), layout[1]);

    let search_input = Paragraph::new(state.search_string.clone());
    frame.render_widget(search_input, Rect::new(
        search_block_area.x,
        search_block_area.y + 1,
        search_block_area.width,
        1,
    ));
}
