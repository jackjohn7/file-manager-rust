use std::env::current_dir;

use utils::files_in_dir;

use crate::state::AppState;

pub enum AppTrigger {
    Refresh
}

pub fn handle_triggers(state: &mut AppState) -> Result<(), &'static str> {
    if let Some(trigger) = &state.trigger {
        match trigger {
            AppTrigger::Refresh => {
                state.files = files_in_dir(current_dir().unwrap().as_path());
                state.scroll_offset = 0;
            }
        }
        state.trigger = None;
    }
    Ok(())
}
