use std::{cmp::Ordering, env::current_dir};

use ratatui::{prelude::*, widgets::*};
use std::fmt::Write;
use utils::FolderItem;

use crate::state::{AppConfig, AppMode, AppState};

fn length(n: usize) -> usize {
    n.checked_ilog10().unwrap_or(0) as usize + 1
}

pub fn list_files<'a>(files: &'a [FolderItem], config: &'a AppConfig) -> Vec<Paragraph<'a>> {
    let mut pg_result = Vec::new();
    for (idx, f) in files.iter().enumerate() {
        let mut result = String::new();
        if config.numbering {
            let mut spaces = String::new();
            for _ in length(idx + 1)..length(files.len()) + 2 {
                spaces.push(' ');
            }
            write!(result, "{}:{}", idx + 1, spaces).unwrap();
        }
        writeln!(
            result,
            "{}",
            match f {
                FolderItem::Directory(dir) => format!(
                    "{}/",
                    match config.show_full_path {
                        true => dir.path.to_str().unwrap().to_string(),
                        false => dir.name_str.clone(),
                    }
                ),
                FolderItem::File(fil) => format!(
                    "{}",
                    match config.show_full_path {
                        true => fil.path.to_str().unwrap().to_string(),
                        false => fil.name_str.clone(),
                    }
                ),
            },
        )
        .unwrap();
        pg_result.push(match f {
            FolderItem::File(_) => Paragraph::new(result),
            FolderItem::Directory(_) => Paragraph::new(result).style(Style::new().green().bold()),
        });
    }
    pg_result
}

pub fn ui(frame: &mut Frame, state: &mut AppState) {
    match state.mode {
        AppMode::Browse => browse_ui(frame, state),
        AppMode::BrowseSearch => browse_search_ui(frame, state),
        AppMode::BrowseCommand => browse_command_ui(frame, state),
    }
}

fn browse_ui(frame: &mut Frame, state: &mut AppState) {
    let paragraphs: Vec<Paragraph> = list_files(&state.files, &state.config)
        .iter()
        .enumerate()
        .map(|(line_idx, pg)| {
            if line_idx == state.line {
                pg.clone().set_style(Style::new().white().on_blue().bold())
            } else {
                pg.clone()
            }
        })
        .collect();

    let block = Block::default()
        .title(format!(
            "Browsing Location: {}",
            current_dir().unwrap().to_str().unwrap()
        ))
        .borders(Borders::ALL);

    frame.render_widget(block.clone(), frame.size());

    let block_area = block.inner(frame.size());
    let paragraph_height = block_area.height.saturating_sub(2); // Adjust for borders
    state.pg_height = Some(paragraph_height as usize);

    // paragraph_height as usize +state.scroll_offset
    for (i, paragraph) in paragraphs[state.scroll_offset
        ..(match state.files.len().cmp(&state.pg_height.unwrap()) {
            Ordering::Greater => state.pg_height.unwrap() + state.scroll_offset,
            _ => state.files.len(),
        })]
        .iter()
        .take(paragraph_height as usize)
        .enumerate()
    {
        frame.render_widget(
            paragraph.clone(),
            Rect::new(block_area.x, block_area.y + i as u16, block_area.width, 1),
        );
    }
}

fn browse_search_ui(frame: &mut Frame, state: &mut AppState) {
    let paragraphs = list_files(&state.files, &state.config);
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(85), Constraint::Percentage(15)])
        .split(frame.size());

    let file_block = Block::default()
        .title(format!(
            "Searching Location: {}",
            current_dir().unwrap().to_str().unwrap()
        ))
        .borders(Borders::ALL);

    frame.render_widget(file_block.clone(), layout[0]);

    let file_block_area = file_block.inner(layout[0]);
    let paragraph_height = file_block_area.height.saturating_sub(2); // Adjust for borders
    state.pg_height = Some(paragraph_height as usize);

    // paragraph_height as usize +state.scroll_offset
    for (i, paragraph) in paragraphs[state.scroll_offset
        ..(match state.files.len().cmp(&state.pg_height.unwrap()) {
            Ordering::Greater => state.pg_height.unwrap() + state.scroll_offset - 1,
            _ => state.files.len(),
        })]
        .iter()
        .take(paragraph_height as usize)
        .enumerate()
    {
        frame.render_widget(
            paragraph.clone(),
            Rect::new(
                file_block_area.x,
                file_block_area.y + i as u16,
                file_block_area.width,
                1,
            ),
        );
    }
    let search_block = Block::default()
        .title("Search (ESC to Cancel, ENTER to Apply and resume browse)")
        .borders(Borders::ALL);
    let search_block_area = search_block.inner(layout[1]);
    frame.render_widget(search_block.clone(), layout[1]);

    let search_input = Paragraph::new(state.search_string.clone());
    frame.render_widget(
        search_input,
        Rect::new(
            search_block_area.x,
            search_block_area.y + 1,
            search_block_area.width,
            1,
        ),
    );
}

fn browse_command_ui(frame: &mut Frame, state: &mut AppState) {
    let paragraphs = list_files(&state.files, &state.config);
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(85), Constraint::Percentage(15)])
        .split(frame.size());

    let file_block = Block::default()
        .title(format!(
            "Executing in Location: {}",
            current_dir().unwrap().to_str().unwrap()
        ))
        .borders(Borders::ALL);

    frame.render_widget(file_block.clone(), layout[0]);

    let file_block_area = file_block.inner(layout[0]);
    let paragraph_height = file_block_area.height.saturating_sub(2); // Adjust for borders
    state.pg_height = Some(paragraph_height as usize);

    // paragraph_height as usize +state.scroll_offset
    for (i, paragraph) in paragraphs[state.scroll_offset
        ..(match state.files.len().cmp(&state.pg_height.unwrap()) {
            Ordering::Greater => state.pg_height.unwrap() + state.scroll_offset - 1,
            _ => state.files.len(),
        })]
        .iter()
        .take(paragraph_height as usize)
        .enumerate()
    {
        frame.render_widget(
            paragraph.clone(),
            Rect::new(
                file_block_area.x,
                file_block_area.y + i as u16,
                file_block_area.width,
                1,
            ),
        );
    }
    let command_block = Block::default()
        .title("Command (ESC to Cancel, ENTER to Execute and resume browse)")
        .borders(Borders::ALL);
    let command_block_area = command_block.inner(layout[1]);
    frame.render_widget(command_block.clone(), layout[1]);

    let command_input = Paragraph::new(state.command_string.clone());
    frame.render_widget(
        command_input,
        Rect::new(
            command_block_area.x,
            command_block_area.y + 1,
            command_block_area.width,
            1,
        ),
    );
}
