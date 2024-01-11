use utils::FolderItem;
use crate::events::triggers::AppTrigger;

pub enum AppMode {
    Browse,
    BrowseSearch,
}

pub struct AppState {
    pub mode: AppMode,
    pub line: usize,
    pub scroll_offset: usize,
    pub files: Vec<FolderItem>,
    pub trigger: Option<AppTrigger>,
    pub pg_height: Option<usize>,
    pub search_string: String,
    pub config: AppConfig
}

#[derive(Default)]
pub struct AppConfig {
    pub numbering: bool,
    pub show_full_path: bool,
}

//impl Default for AppConfig {
//    fn default() -> Self {
//        Self { numbering: false, show_full_path: false }
//    }
//}
