use crate::errors::AppResult;
use serde::Serialize;
use walkdir::WalkDir;
use std::fs;

#[derive(Serialize, Clone)]
pub struct SearchResult {
    file: String,
    line: usize,
    content: String,
}

#[tauri::command]
pub async fn search_in_files(
    query: String,
    path: String,
    _include: Option<String>, // TODO: Implement filtering
    _exclude: Option<String>,
) -> AppResult<Vec<SearchResult>> {
    let mut results = Vec::new();

    // Basic implementation: Walk all files, read content, find substring
    // Optimization: This is slow for large projects. Future: Use `grep` or `ignore` crate.
    for entry in WalkDir::new(&path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let file_path = entry.path().to_string_lossy().to_string();
            // Skip binary files / basic heuristic check could be here
            if let Ok(content) = fs::read_to_string(entry.path()) {
                for (i, line) in content.lines().enumerate() {
                    if line.contains(&query) {
                        results.push(SearchResult {
                            file: file_path.clone(),
                            line: i + 1,
                            content: line.trim().to_string(),
                        });
                        if results.len() >= 100 { break; } // Limit results
                    }
                }
            }
        }
        if results.len() >= 100 { break; }
    }

    Ok(results)
}
