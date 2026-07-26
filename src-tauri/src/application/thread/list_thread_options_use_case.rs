use std::path::Path;

use crate::application::thread::dto::ThreadOptionsOutput;
use crate::application::thread::repository;

pub struct ListThreadOptionsUseCase;

impl ListThreadOptionsUseCase {
    pub fn execute(db_path: &Path) -> Result<ThreadOptionsOutput, String> {
        repository::list_thread_options(db_path)
    }
}
