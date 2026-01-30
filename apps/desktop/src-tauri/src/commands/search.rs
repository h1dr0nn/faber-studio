use crate::errors::AppResult;
use glob::Pattern;
use serde::Serialize;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Serialize, Clone)]
pub struct SearchResult {
    file: String,
    line: usize,
    content: String,
}

/// Parse comma-separated glob patterns into a Vec of Pattern
fn parse_patterns(input: &Option<String>) -> Vec<Pattern> {
    match input {
        Some(s) if !s.trim().is_empty() => s
            .split(',')
            .filter_map(|p| {
                let trimmed = p.trim();
                if trimmed.is_empty() {
                    return None;
                }
                Pattern::new(trimmed).ok()
            })
            .collect(),
        _ => Vec::new(),
    }
}

/// Check if a file path matches any of the given patterns
fn matches_any_pattern(file_path: &Path, patterns: &[Pattern]) -> bool {
    if patterns.is_empty() {
        return false;
    }

    let file_name = file_path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    let path_str = file_path.to_string_lossy();

    patterns.iter().any(|p| {
        // Match against filename (e.g., *.ts)
        p.matches(file_name)
            // Or match against full path for patterns like src/**
            || p.matches(&path_str)
            // Or match against path with forward slashes for cross-platform
            || p.matches(&path_str.replace('\\', "/"))
    })
}

#[tauri::command]
pub async fn search_in_files(
    query: String,
    path: String,
    include: Option<String>,
    exclude: Option<String>,
) -> AppResult<Vec<SearchResult>> {
    let mut results = Vec::new();

    log::info!(
        "Searching for '{}' in '{}' (include: {:?}, exclude: {:?})",
        query,
        path,
        include,
        exclude
    );

    // Parse glob patterns
    let include_patterns = parse_patterns(&include);
    let exclude_patterns = parse_patterns(&exclude);

    // Directories to always skip
    let skip_dirs = [
        "node_modules",
        ".git",
        "target",
        "dist",
        "build",
        ".svelte-kit",
        ".next",
        "__pycache__",
    ];

    // Binary extensions to skip
    let binary_exts = [
        "png", "jpg", "jpeg", "gif", "ico", "icns", "woff", "woff2", "ttf", "eot", "exe", "dll",
        "so", "dylib", "pdf", "zip", "tar", "gz", "webp", "svg", "mp3", "mp4", "wav", "avi",
    ];

    for entry in WalkDir::new(&path)
        .into_iter()
        .filter_entry(|e| {
            let name = e.file_name().to_string_lossy();
            !skip_dirs.iter().any(|d| name == *d)
        })
        .filter_map(|e| e.ok())
    {
        if !entry.file_type().is_file() {
            continue;
        }

        let file_path = entry.path();

        // Skip binary files by extension
        let ext = file_path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");
        if binary_exts.contains(&ext.to_lowercase().as_str()) {
            continue;
        }

        // Apply include filter: if patterns exist, file must match at least one
        if !include_patterns.is_empty() && !matches_any_pattern(file_path, &include_patterns) {
            continue;
        }

        // Apply exclude filter: skip if matches any exclude pattern
        if matches_any_pattern(file_path, &exclude_patterns) {
            continue;
        }

        let file_path_str = file_path.to_string_lossy().to_string();

        if let Ok(content) = fs::read_to_string(file_path) {
            for (i, line) in content.lines().enumerate() {
                if line.contains(&query) {
                    results.push(SearchResult {
                        file: file_path_str.clone(),
                        line: i + 1,
                        content: line.trim().to_string(),
                    });
                    if results.len() >= 200 {
                        break;
                    }
                }
            }
        }

        if results.len() >= 200 {
            break;
        }
    }

    log::info!("Found {} results", results.len());
    Ok(results)
}
