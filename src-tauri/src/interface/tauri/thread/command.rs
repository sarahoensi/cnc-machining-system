use std::path::{Path, PathBuf};
use tauri::command;
use tauri::Manager;

use crate::application::{ListThreadOptionsUseCase, SolveThreadInput, SolveThreadUseCase};
use crate::interface::tauri::error::TauriError;

use super::{SolveThreadRequest, SolveThreadResponse, ThreadOptionsResponse};

const DB_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/data/threads.sqlite");
const MISSING_DATABASE_MESSAGE: &str =
    "Thread database not found. Run the thread import script first.";

fn bundled_or_development_db_path(app: &tauri::AppHandle) -> PathBuf {
    app.path()
        .resource_dir()
        .ok()
        .map(|dir| dir.join("data").join("threads.sqlite"))
        .filter(|path| path.exists())
        .unwrap_or_else(|| Path::new(DB_PATH).to_path_buf())
}

#[command]
pub fn list_thread_options(app: tauri::AppHandle) -> Result<ThreadOptionsResponse, TauriError> {
    let db_path = bundled_or_development_db_path(&app);
    list_thread_options_from_path(&db_path)
}

#[command]
pub fn solve_thread(
    app: tauri::AppHandle,
    request: SolveThreadRequest,
) -> Result<SolveThreadResponse, TauriError> {
    let db_path = bundled_or_development_db_path(&app);
    solve_thread_from_path(&db_path, request)
}

pub fn list_thread_options_from_path(db_path: &Path) -> Result<ThreadOptionsResponse, TauriError> {
    if !db_path.exists() {
        return Err(TauriError::message(MISSING_DATABASE_MESSAGE));
    }

    ListThreadOptionsUseCase::execute(db_path)
        .map(Into::into)
        .map_err(TauriError::message)
}

pub fn solve_thread_from_path(
    db_path: &Path,
    request: SolveThreadRequest,
) -> Result<SolveThreadResponse, TauriError> {
    if !db_path.exists() {
        return Err(TauriError::message(MISSING_DATABASE_MESSAGE));
    }

    let input: SolveThreadInput = request.into();
    let output = SolveThreadUseCase::execute(db_path, input).map_err(TauriError::message)?;

    Ok(output.into())
}
