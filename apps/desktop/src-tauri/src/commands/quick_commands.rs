use crate::errors::AppResult;
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectInfo {
    pub package_manager: String,
    pub project_type: String,
    pub has_tauri: bool,
    pub has_android: bool,
    pub has_ios: bool,
    pub has_node_modules: bool,
}

/// Detect the package manager used in the project
#[tauri::command]
pub async fn detect_package_manager(path: String) -> AppResult<String> {
    let mut current_path = Path::new(&path).to_path_buf();
    
    // Walk up the directory tree to find lock files (for monorepo support)
    loop {
        // Check for lock files in order of priority
        if current_path.join("bun.lockb").exists() {
            return Ok("bun".to_string());
        }
        if current_path.join("pnpm-lock.yaml").exists() {
            return Ok("pnpm".to_string());
        }
        if current_path.join("yarn.lock").exists() {
            return Ok("yarn".to_string());
        }
        if current_path.join("package-lock.json").exists() {
            return Ok("npm".to_string());
        }
        
        // Move to parent directory
        if let Some(parent) = current_path.parent() {
            current_path = parent.to_path_buf();
        } else {
            break;
        }
        
        // Stop if we've gone too far up (e.g., root)
        if !current_path.join("package.json").exists() && !current_path.join("pnpm-workspace.yaml").exists() {
            break;
        }
    }
    
    // Fallback to npm if package.json exists in original path
    if Path::new(&path).join("package.json").exists() {
        return Ok("npm".to_string());
    }
    
    Ok("unknown".to_string())
}

/// Detect project type and capabilities
#[tauri::command]
pub async fn detect_project_info(path: String) -> AppResult<ProjectInfo> {
    let project_path = Path::new(&path);
    
    // Detect package manager - walk up to find lock files
    let package_manager = {
        let mut current = project_path.to_path_buf();
        let mut pm = "npm".to_string();
        loop {
            if current.join("bun.lockb").exists() {
                pm = "bun".to_string();
                break;
            } else if current.join("pnpm-lock.yaml").exists() {
                pm = "pnpm".to_string();
                break;
            } else if current.join("yarn.lock").exists() {
                pm = "yarn".to_string();
                break;
            } else if current.join("package-lock.json").exists() {
                pm = "npm".to_string();
                break;
            }
            
            if let Some(parent) = current.parent() {
                if !parent.join("package.json").exists() && !parent.join("pnpm-workspace.yaml").exists() {
                    break;
                }
                current = parent.to_path_buf();
            } else {
                break;
            }
        }
        pm
    };
    
    // Check for Tauri
    let has_tauri = project_path.join("src-tauri").exists() 
        || project_path.join("tauri.conf.json").exists();
    
    // Check for Android
    let has_android = project_path.join("src-tauri/gen/android").exists();
    
    // Check for iOS
    let has_ios = project_path.join("src-tauri/gen/apple").exists();
    
    // Determine project type
    let project_type = if has_tauri {
        "tauri"
    } else if project_path.join("next.config.js").exists() || project_path.join("next.config.mjs").exists() {
        "nextjs"
    } else if project_path.join("vite.config.ts").exists() || project_path.join("vite.config.js").exists() {
        "vite"
    } else if project_path.join("package.json").exists() {
        "node"
    } else if project_path.join("Cargo.toml").exists() {
        "rust"
    } else {
        "unknown"
    }.to_string();
    
    // Check if node_modules exists
    let has_node_modules = project_path.join("node_modules").exists();
    
    Ok(ProjectInfo {
        package_manager,
        project_type,
        has_tauri,
        has_android,
        has_ios,
        has_node_modules,
    })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AndroidVersion {
    pub version_code: i32,
    pub version_name: String,
}

/// Read Android version from build.gradle.kts
#[tauri::command]
pub async fn read_android_version(path: String) -> AppResult<AndroidVersion> {
    let gradle_path = Path::new(&path).join("src-tauri/gen/android/app/build.gradle.kts");
    
    if !gradle_path.exists() {
        return Err(crate::errors::AppError::NotFound("Android build.gradle.kts not found".to_string()));
    }
    
    let content = fs::read_to_string(&gradle_path)?;
    
    let mut version_code = 1;
    let mut version_name = "1.0.0".to_string();
    
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("versionCode") {
            if let Some(val) = trimmed.split('=').nth(1) {
                version_code = val.trim().parse().unwrap_or(1);
            }
        } else if trimmed.starts_with("versionName") {
            if let Some(val) = trimmed.split('=').nth(1) {
                version_name = val.trim().trim_matches('"').to_string();
            }
        }
    }
    
    Ok(AndroidVersion { version_code, version_name })
}

/// Update Android version (increment versionCode and versionName patch)
#[tauri::command]
pub async fn update_android_version(path: String) -> AppResult<AndroidVersion> {
    let gradle_path = Path::new(&path).join("src-tauri/gen/android/app/build.gradle.kts");
    
    if !gradle_path.exists() {
        return Err(crate::errors::AppError::NotFound("Android build.gradle.kts not found".to_string()));
    }
    
    let content = fs::read_to_string(&gradle_path)?;
    let mut new_content = String::new();
    let mut new_version_code = 1;
    let mut new_version_name = "1.0.0".to_string();
    
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("versionCode") {
            if let Some(val) = trimmed.split('=').nth(1) {
                let current: i32 = val.trim().parse().unwrap_or(1);
                new_version_code = current + 1;
                new_content.push_str(&format!("        versionCode = {}\n", new_version_code));
            } else {
                new_content.push_str(line);
                new_content.push('\n');
            }
        } else if trimmed.starts_with("versionName") {
            if let Some(val) = trimmed.split('=').nth(1) {
                let current = val.trim().trim_matches('"');
                // Increment patch version (e.g., 1.0.0 -> 1.0.1)
                let parts: Vec<&str> = current.split('.').collect();
                if parts.len() == 3 {
                    let major = parts[0];
                    let minor = parts[1];
                    let patch: i32 = parts[2].parse().unwrap_or(0) + 1;
                    new_version_name = format!("{}.{}.{}", major, minor, patch);
                } else {
                    new_version_name = current.to_string();
                }
                new_content.push_str(&format!("        versionName = \"{}\"\n", new_version_name));
            } else {
                new_content.push_str(line);
                new_content.push('\n');
            }
        } else {
            new_content.push_str(line);
            new_content.push('\n');
        }
    }
    
    fs::write(&gradle_path, new_content)?;
    
    Ok(AndroidVersion { version_code: new_version_code, version_name: new_version_name })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArtifactInfo {
    pub path: String,
    pub name: String,
    pub extension: String,
    pub artifact_type: String, // "apk" | "aab"
}

/// Find Android artifacts (.apk and .aab) in search path
#[tauri::command]
pub async fn find_android_artifacts(path: String) -> AppResult<Vec<ArtifactInfo>> {
    let mut artifacts = Vec::new();
    let search_root = Path::new(&path).join("src-tauri/gen/android/app/build/outputs");
    
    if !search_root.exists() {
        return Ok(artifacts);
    }

    fn walk_dir(dir: &Path, artifacts: &mut Vec<ArtifactInfo>) -> std::io::Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    walk_dir(&path, artifacts)?;
                } else if let Some(ext) = path.extension() {
                    let extension = ext.to_string_lossy().to_lowercase();
                    if extension == "apk" || extension == "aab" {
                        let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                        
                        // We filter for "release" and "unsigned" logic similar to the batch script
                        // For simplicity, we return everything and let frontend decide or we can filter here
                        artifacts.push(ArtifactInfo {
                            path: path.to_string_lossy().to_string(),
                            name,
                            extension: extension.clone(),
                            artifact_type: extension,
                        });
                    }
                }
            }
        }
        Ok(())
    }

    let _ = walk_dir(&search_root, &mut artifacts);
    
    Ok(artifacts)
}

/// Copy and rename build artifact to builds folder
#[tauri::command]
pub async fn manage_build_artifact(
    project_root: String,
    source_path: String,
    target_name: String
) -> AppResult<String> {
    let builds_dir = Path::new(&project_root).join("builds");
    if !builds_dir.exists() {
        fs::create_dir_all(&builds_dir)?;
    }

    let source = Path::new(&source_path);
    if !source.exists() {
        return Err(crate::errors::AppError::NotFound(format!("Source artifact not found: {}", source_path)));
    }

    let target_path = builds_dir.join(target_name);
    fs::copy(source, &target_path)?;

    Ok(target_path.to_string_lossy().to_string())
}

/// Open Xcode project (macOS only)
#[tauri::command]
pub async fn open_xcode_project(_path: String) -> AppResult<()> {
    #[cfg(target_os = "macos")]
    {
        let xcode_path = Path::new(&_path).join("src-tauri/gen/apple");
        if !xcode_path.exists() {
            return Err(crate::errors::AppError::NotFound("iOS project not found".to_string()));
        }
        
        // Find .xcworkspace or .xcodeproj
        let workspace = xcode_path.join("app.xcworkspace");
        let project = xcode_path.join("app.xcodeproj");
        
        let target = if workspace.exists() {
            workspace
        } else if project.exists() {
            project
        } else {
            return Err(crate::errors::AppError::NotFound("Xcode project not found".to_string()));
        };
        
        std::process::Command::new("open")
            .arg(target)
            .spawn()?;
        
        return Ok(());
    }
    
    #[cfg(not(target_os = "macos"))]
    {
        Err(crate::errors::AppError::NotSupported("Opening Xcode is only supported on macOS".to_string()))
    }
}
