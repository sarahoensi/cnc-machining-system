use std::path::Path;

use crate::application::thread::dto::{SolveThreadInput, SolveThreadOutput};
use crate::application::thread::repository;

pub struct SolveThreadUseCase;

impl SolveThreadUseCase {
    pub fn execute(db_path: &Path, input: SolveThreadInput) -> Result<SolveThreadOutput, String> {
        repository::solve_thread(db_path, &input.thread_type, &input.size, &input.pitch)
    }
}
