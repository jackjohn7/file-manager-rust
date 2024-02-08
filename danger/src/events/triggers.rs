use std::env::current_dir;

use utils::{files_in_dir, FolderItem};

use crate::state::AppState;

pub enum AppTrigger {
    Refresh
}

pub fn handle_triggers(state: &mut AppState) -> Result<(), &'static str> {
    if let Some(trigger) = &state.trigger {
        match trigger {
            AppTrigger::Refresh => {
                state.files = files_in_dir(current_dir().unwrap().as_path())
                    .into_iter()
                    .filter(|file|
                        match file {
                            FolderItem::Directory(dir) => dir.name_str.contains(&state.search_string),
                            FolderItem::File(fil) => fil.name_str.contains(&state.search_string),
                        }
                    )
                    .collect();
                state.line = 0;
                state.scroll_offset = 0;
            }
        }
        state.trigger = None;
    }
    Ok(())
}
