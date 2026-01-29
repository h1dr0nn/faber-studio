use crate::models::FileNode;
use crate::errors::AppResult;
use std::path::Path;


#[tauri::command]
pub async fn scan_project(path: String) -> AppResult<FileNode> {
    let root_path = Path::new(&path);
    if !root_path.exists() {
        return Err(crate::errors::AppError::NotFound(format!("Path not found: {}", path)));
    }

    Ok(build_tree(root_path)?)
}

fn build_tree(path: &Path) -> std::io::Result<FileNode> {
    let name = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("project")
        .to_string();

    let is_dir = path.is_dir();
    let mut children = if is_dir { Some(vec![]) } else { None };

    if is_dir {
        // Read directory and recurse
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            let entry_name = entry_path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("");

            // Filter out common ignored directories to keep it performant
            if entry_name.starts_with('.') || entry_name == "node_modules" || entry_name == "target" || entry_name == "dist" {
                continue;
            }

            children.as_mut().unwrap().push(build_tree(&entry_path)?);
        }
    }

    Ok(FileNode {
        name,
        path: path.to_str().unwrap_or("").to_string(),
        is_dir,
        children,
        size: if !is_dir { path.metadata()?.len() } else { 0 },
    })
}


#[tauri::command]
pub async fn read_file(path: String) -> AppResult<String> {
    let content = std::fs::read_to_string(path)?;
    Ok(content)
}

#[tauri::command]
pub async fn write_file(path: String, content: String) -> AppResult<()> {
    std::fs::write(path, content)?;
    Ok(())
}

#[tauri::command]
pub async fn create_file(path: String) -> AppResult<()> {
    std::fs::File::create(path)?;
    Ok(())
}

#[tauri::command]
pub async fn create_dir(path: String) -> AppResult<()> {
    std::fs::create_dir_all(path)?;
    Ok(())
}

#[tauri::command]
pub async fn delete_path(path: String) -> AppResult<()> {
    let metadata = std::fs::metadata(&path)?;
    if metadata.is_dir() {
        std::fs::remove_dir_all(path)?;
    } else {
        std::fs::remove_file(path)?;
    }
    Ok(())
}

#[tauri::command]
pub async fn rename_path(old_path: String, new_path: String) -> AppResult<()> {
    std::fs::rename(old_path, new_path)?;
    Ok(())
}



