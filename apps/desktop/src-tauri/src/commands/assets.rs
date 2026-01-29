use crate::models::FileNode;
use crate::errors::AppResult;

#[tauri::command]
pub async fn get_assets() -> AppResult<Vec<FileNode>> {
    // Phase 4 will implement asset scanning
    Ok(vec![])
}
