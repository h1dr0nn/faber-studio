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

    log::info!("Searching for '{}' in '{}'", query, path);

    // Directories to skip
    let skip_dirs = ["node_modules", ".git", "target", "dist", "build", ".svelte-kit", ".next", "__pycache__"];

    // Basic implementation: Walk all files, read content, find substring
    for entry in WalkDir::new(&path)
        .into_iter()
        .filter_entry(|e| {
            // Skip common non-text directories
            let name = e.file_name().to_string_lossy();
            !skip_dirs.iter().any(|d| name == *d)
        })
        .filter_map(|e| e.ok()) 
    {
        if entry.file_type().is_file() {
            let file_path = entry.path().to_string_lossy().to_string();
            
            // Skip binary files by extension
            let ext = entry.path().extension().and_then(|e| e.to_str()).unwrap_or("");
            let binary_exts = ["png", "jpg", "jpeg", "gif", "ico", "icns", "woff", "woff2", "ttf", "eot", "exe", "dll", "so", "dylib", "pdf", "zip", "tar", "gz"];
            if binary_exts.contains(&ext.to_lowercase().as_str()) {
                continue;
            }
            
            if let Ok(content) = fs::read_to_string(entry.path()) {
                for (i, line) in content.lines().enumerate() {
                    if line.contains(&query) {
                        results.push(SearchResult {
                            file: file_path.clone(),
                            line: i + 1,
                            content: line.trim().to_string(),
                        });
                        if results.len() >= 200 { break; } // Limit results
                    }
                }
            }
        }
        if results.len() >= 200 { break; }
    }

    log::info!("Found {} results", results.len());
    Ok(results)
}
