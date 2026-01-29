use crate::models::FileNode;
use crate::errors::{AppError, AppResult};
use std::path::{Path, PathBuf};
use image::{DynamicImage, ImageFormat};
use std::fs::File;
use std::io::BufWriter;
use walkdir::WalkDir;

#[tauri::command]
pub async fn get_assets() -> AppResult<Vec<FileNode>> {
    // Phase 4 will implement asset scanning
    Ok(vec![])
}

/// Find the icons directory in a Tauri project
#[tauri::command]
pub async fn find_icons_dir(project_root: String) -> AppResult<Option<String>> {
    let root = Path::new(&project_root);
    
    // Common path patterns for Tauri icons directory (using proper path joining)
    let path_patterns: Vec<PathBuf> = vec![
        root.join("src-tauri").join("icons"),
        root.join("src-tauri").join("icon"),
        root.join("tauri").join("icons"),
        root.join("icons"),
        // Monorepo patterns
        root.join("apps").join("desktop").join("src-tauri").join("icons"),
        root.join("packages").join("desktop").join("src-tauri").join("icons"),
        root.join("app").join("src-tauri").join("icons"),
    ];
    
    // First try direct paths
    for path in &path_patterns {
        if path.exists() && path.is_dir() {
            return Ok(Some(path.to_string_lossy().to_string()));
        }
    }
    
    // If not found, search recursively (max depth 5 for monorepos)
    for entry in WalkDir::new(root).max_depth(5).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_dir() {
            let name = path.file_name().unwrap_or_default().to_string_lossy();
            if name == "icons" {
                // Check if parent is src-tauri or tauri
                if let Some(parent) = path.parent() {
                    let parent_name = parent.file_name().unwrap_or_default().to_string_lossy();
                    if parent_name == "src-tauri" || parent_name == "tauri" {
                        return Ok(Some(path.to_string_lossy().to_string()));
                    }
                }
            }
        }
    }
    
    Ok(None)
}

#[tauri::command]
pub async fn generate_icons(
    source_path: String,
    target_dir: String,
    platforms: Vec<String>,
) -> AppResult<String> {
    let source = Path::new(&source_path);
    let target = Path::new(&target_dir);

    if !source.exists() {
        return Err(AppError::Internal("Source file does not exist".into()));
    }

    // Load the image
    let img = image::open(source)
        .map_err(|e| AppError::Internal(format!("Failed to open image: {}", e)))?;

    // Create target directory if it doesn't exist
    if !target.exists() {
        std::fs::create_dir_all(target)
            .map_err(|e| AppError::Internal(format!("Failed to create target directory: {}", e)))?;
    }

    // Generate Windows ICO
    if platforms.contains(&"desktop".to_string()) {
        generate_ico(&img, target)?;
        generate_icns(&img, target)?;
    }

    // Generate Mobile Icons (simplified placeholder for now)
    if platforms.contains(&"ios".to_string()) || platforms.contains(&"android".to_string()) {
        generate_mobile_pngs(&img, target)?;
    }

    Ok("Icons generated successfully".into())
}

fn generate_ico(img: &DynamicImage, output_dir: &Path) -> AppResult<()> {
    let sizes = vec![16, 24, 32, 48, 64, 128, 256];
    let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);

    for &size in &sizes {
        let resized = img.resize_exact(size, size, image::imageops::FilterType::Lanczos3);
        let rgba = resized.to_rgba8();
        let image = ico::IconImage::from_rgba_data(size, size, rgba.into_raw());
        icon_dir.add_entry(ico::IconDirEntry::encode(&image).map_err(|e| AppError::Internal(e.to_string()))?);
    }

    let file = File::create(output_dir.join("icon.ico"))
        .map_err(|e| AppError::Internal(format!("Failed to create ICO: {}", e)))?;
    let mut writer = BufWriter::new(file);
    icon_dir.write(&mut writer).map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(())
}

fn generate_icns(img: &DynamicImage, output_dir: &Path) -> AppResult<()> {
    // Simple ICNS generation using the icns crate
    let mut family = icns::IconFamily::new();
    let sizes = vec![16, 32, 64, 128, 256, 512, 1024];

    for &size in &sizes {
        let resized = img.resize_exact(size, size, image::imageops::FilterType::Lanczos3);
        let rgba = resized.to_rgba8();
        let image = icns::Image::from_data(icns::PixelFormat::RGBA, size, size, rgba.into_raw())
            .map_err(|e| AppError::Internal(e.to_string()))?;
        family.add_icon(&image).map_err(|e| AppError::Internal(e.to_string()))?;
    }

    let file = File::create(output_dir.join("icon.icns"))
        .map_err(|e| AppError::Internal(format!("Failed to create ICNS: {}", e)))?;
    let mut writer = BufWriter::new(file);
    family.write(&mut writer).map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(())
}

fn generate_mobile_pngs(img: &DynamicImage, output_dir: &Path) -> AppResult<()> {
    let sizes = vec![512, 1024];
    for &size in &sizes {
        let resized = img.resize_exact(size, size, image::imageops::FilterType::Lanczos3);
        let path = output_dir.join(format!("icon_{}x{}.png", size, size));
        resized.save_with_format(path, ImageFormat::Png)
            .map_err(|e| AppError::Internal(format!("Failed to save mobile PNG: {}", e)))?;
    }
    Ok(())
}
