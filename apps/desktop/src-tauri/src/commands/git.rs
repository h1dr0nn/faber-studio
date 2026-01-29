use crate::errors::AppResult;
use serde::Serialize;
use std::process::Command;

#[derive(Serialize, Clone, Debug)]
pub struct FileStatus {
    pub path: String,
    pub status: String, // e.g. "M", "A", "??"
    pub staged: bool,
}

#[derive(Serialize, Clone, Debug)]
pub struct GitStatusResult {
    pub staged: Vec<FileStatus>,
    pub unstaged: Vec<FileStatus>,
}

#[tauri::command]
pub async fn git_status(path: String) -> AppResult<GitStatusResult> {
    // Run git status --porcelain
    let output = Command::new("git")
        .args(&["status", "--porcelain"])
        .current_dir(&path)
        .output()?;
        
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    let mut staged = Vec::new();
    let mut unstaged = Vec::new();

    for line in stdout.lines() {
        if line.len() < 4 { continue; }
        
        let x = line.chars().nth(0).unwrap_or(' ');
        let y = line.chars().nth(1).unwrap_or(' ');
        let path_str = line[3..].trim().to_string();

        // X = index status, Y = working tree status
        // ?? = untracked
        
        if x != ' ' && x != '?' {
            staged.push(FileStatus {
                path: path_str.clone(),
                status: x.to_string(),
                staged: true,
            });
        }
        
        if y != ' ' {
            unstaged.push(FileStatus {
                path: path_str.clone(),
                status: if x == '?' && y == '?' { "??".to_string() } else { y.to_string() },
                staged: false,
            });
        }
    }

    Ok(GitStatusResult { staged, unstaged })
}

#[tauri::command]
pub async fn git_stage(path: String, file: String) -> AppResult<()> {
    Command::new("git")
        .args(&["add", &file])
        .current_dir(path)
        .output()?;
    Ok(())
}

#[tauri::command]
pub async fn git_unstage(path: String, file: String) -> AppResult<()> {
    Command::new("git")
        .args(&["reset", "HEAD", &file])
        .current_dir(path)
        .output()?;
    Ok(())
}

#[tauri::command]
pub async fn git_commit(path: String, message: String) -> AppResult<()> {
    Command::new("git")
        .args(&["commit", "-m", &message])
        .current_dir(path)
        .output()?;
    Ok(())
}

#[tauri::command]
pub async fn git_diff_staged(path: String) -> AppResult<String> {
    let output = Command::new("git")
        .args(&["diff", "--staged"])
        .current_dir(path)
        .output()?;
    
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

#[tauri::command]
pub async fn git_push(path: String) -> AppResult<()> {
    Command::new("git")
        .args(&["push"])
        .current_dir(path)
        .output()?;
    Ok(())
}

#[tauri::command]
pub async fn git_pull(path: String) -> AppResult<()> {
    Command::new("git")
        .args(&["pull"])
        .current_dir(path)
        .output()?;
    Ok(())
}

#[tauri::command]
pub async fn git_discard_changes(path: String, file: String) -> AppResult<()> {
    Command::new("git")
        .args(&["restore", &file])
        .current_dir(path)
        .output()?;
    Ok(())
}

#[tauri::command]
pub async fn git_branch(path: String) -> AppResult<String> {
    let output = Command::new("git")
        .args(&["rev-parse", "--abbrev-ref", "HEAD"])
        .current_dir(path)
        .output()?;
        
    let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(branch)
}
