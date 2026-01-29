use crate::models::FileNode;
use crate::errors::{AppError, AppResult};
use std::path::{Path, PathBuf};
use image::DynamicImage;
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

use std::process::Command;

#[tauri::command]
pub async fn generate_icons(
    source_path: String,
    target_dir: String,
    platforms: Vec<String>,
    apply_squircle: bool,
) -> AppResult<String> {
    let source = Path::new(&source_path);
    let target = Path::new(&target_dir);

    if !source.exists() {
        return Err(AppError::Internal("Source file does not exist".into()));
    }

    // Load image for custom processing
    let img = image::open(source)
        .map_err(|e| AppError::Internal(format!("Failed to open image: {}", e)))?;

    // Create target directory if needed
    if !target.exists() {
        std::fs::create_dir_all(target)
            .map_err(|e| AppError::Internal(format!("Failed to create target directory: {}", e)))?;
    }

    // Step 1: Run Tauri CLI for standard desktop PNGs
    let mut cmd = Command::new("pnpm");
    cmd.args(["tauri", "icon", &source_path, "--output", &target_dir]);
    
    let output = cmd.output().or_else(|_| {
        Command::new("npx")
            .args(["tauri", "icon", &source_path, "--output", &target_dir])
            .output()
    }).map_err(|e| AppError::Internal(format!("Failed to execute tauri icon CLI: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(AppError::Internal(format!("Tauri CLI failed: {}", stderr)));
    }

    // Step 2: Post-process for "Native" look on macOS/Windows if requested
    if platforms.contains(&"desktop".to_string()) {
        if apply_squircle {
            generate_native_icns(&img, target)?;
            generate_native_ico(&img, target)?;
        }
    }

    // Step 3: Move mobile icons to proper structure if requested
    if platforms.contains(&"ios".to_string()) {
        move_ios_icons(target)?;
    }
    if platforms.contains(&"android".to_string()) {
        move_android_icons(target)?;
    }

    Ok("Icons generated successfully".into())
}

/// Move iOS icons from flat structure to gen/apple/
fn move_ios_icons(base_dir: &Path) -> AppResult<()> {
    // Standard path: sibling to 'icons' or in base_dir
    let parent = if base_dir.file_name().map_or(false, |n| n == "icons") {
        base_dir.parent().unwrap_or(base_dir)
    } else {
        base_dir
    };
    
    let apple_dir = parent.join("gen").join("apple");
    std::fs::create_dir_all(&apple_dir)
        .map_err(|e| AppError::Internal(format!("Failed to create gen/apple in {:?}: {}", parent, e)))?;

    // Move all AppIcon-*.png files
    for entry in std::fs::read_dir(base_dir).map_err(|e| AppError::Internal(e.to_string()))? {
        let entry = entry.map_err(|e| AppError::Internal(e.to_string()))?;
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with("AppIcon-") && name.ends_with(".png") {
            let dest = apple_dir.join(&name);
            std::fs::rename(entry.path(), dest)
                .map_err(|e| AppError::Internal(format!("Failed to move {}: {}", name, e)))?;
        }
    }
    Ok(())
}

/// Move Android icons from flat structure to gen/android/mipmap-*/
fn move_android_icons(base_dir: &Path) -> AppResult<()> {
    // Standard path: sibling to 'icons' or in base_dir
    let parent = if base_dir.file_name().map_or(false, |n| n == "icons") {
        base_dir.parent().unwrap_or(base_dir)
    } else {
        base_dir
    };
    
    let android_dir = parent.join("gen").join("android");
    
    // Android mipmap densities
    let densities = ["mdpi", "hdpi", "xhdpi", "xxhdpi", "xxxhdpi"];
    for density in densities {
        let mipmap_dir = android_dir.join(format!("mipmap-{}", density));
        std::fs::create_dir_all(&mipmap_dir)
            .map_err(|e| AppError::Internal(format!("Failed to create mipmap-{}: {}", density, e)))?;
    }

    // Move mipmap-*/*.png files
    for entry in std::fs::read_dir(base_dir).map_err(|e| AppError::Internal(e.to_string()))? {
        let entry = entry.map_err(|e| AppError::Internal(e.to_string()))?;
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with("mipmap-") && entry.path().is_dir() {
            // This is a mipmap folder from tauri icon, move its contents
            let src_mipmap = entry.path();
            let dest_mipmap = android_dir.join(&name);
            std::fs::create_dir_all(&dest_mipmap)
                .map_err(|e| AppError::Internal(format!("Failed to create {}: {}", name, e)))?;
            
            for file in std::fs::read_dir(&src_mipmap).map_err(|e| AppError::Internal(e.to_string()))? {
                let file = file.map_err(|e| AppError::Internal(e.to_string()))?;
                let file_name = file.file_name();
                let dest = dest_mipmap.join(&file_name);
                std::fs::rename(file.path(), dest)
                    .map_err(|e| AppError::Internal(format!("Failed to move {:?}: {}", file_name, e)))?;
            }
            // Remove empty source mipmap folder
            let _ = std::fs::remove_dir(&src_mipmap);
        }
    }
    Ok(())
}


fn apply_mac_mask(img: &DynamicImage, size: u32) -> DynamicImage {
    // macOS Design Guidelines:
    // Content area is approx 80-82% of the icon size
    let content_size = (size as f32 * 0.82) as u32;
    let padding = (size - content_size) / 2;
    
    // Resize content
    let resized_content = img.resize_exact(content_size, content_size, image::imageops::FilterType::Lanczos3);
    
    // Create transparent canvas
    let mut canvas = image::ImageBuffer::new(size, size);
    
    // Apply squircle mask (approx with rounded rect)
    // Radius for macOS is ~22.3% of the size
    let radius = (content_size as f32 * 0.223) as f32;
    
    let content_rgba = resized_content.to_rgba8();
    
    for y in 0..content_size {
        for x in 0..content_size {
            let px = content_rgba.get_pixel(x, y);
            
            // Simple rounded rect mask check
            let dx = if x < radius as u32 { radius - x as f32 }
                     else if x > content_size - radius as u32 { x as f32 - (content_size as f32 - radius) }
                     else { 0.0 };
            let dy = if y < radius as u32 { radius - y as f32 }
                     else if y > content_size - radius as u32 { y as f32 - (content_size as f32 - radius) }
                     else { 0.0 };
            
            let mut alpha = px[3];
            if dx > 0.0 && dy > 0.0 {
                let dist = (dx*dx + dy*dy).sqrt();
                if dist > radius {
                    alpha = 0;
                } else if dist > radius - 1.0 {
                    // Anti-aliasing
                    let aa = (radius - dist).clamp(0.0, 1.0);
                    alpha = (alpha as f32 * aa) as u8;
                }
            }
            
            canvas.put_pixel(x + padding, y + padding, image::Rgba([px[0], px[1], px[2], alpha]));
        }
    }
    
    DynamicImage::ImageRgba8(canvas)
}

fn generate_native_icns(img: &DynamicImage, output_dir: &Path) -> AppResult<()> {
    let mut family = icns::IconFamily::new();
    let sizes = vec![16, 32, 64, 128, 256, 512, 1024];

    for &size in &sizes {
        let masked = apply_mac_mask(img, size);
        let rgba = masked.to_rgba8();
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

fn generate_native_ico(img: &DynamicImage, output_dir: &Path) -> AppResult<()> {
    let sizes = vec![16, 24, 32, 48, 64, 128, 256];
    let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);

    for &size in &sizes {
        // Windows icons usually don't have the squircle mask but padding is good
        let content_size = (size as f32 * 0.9) as u32; // Less padding for Windows
        let padding = (size - content_size) / 2;
        let resized = img.resize_exact(content_size, content_size, image::imageops::FilterType::Lanczos3);
        
        let mut canvas = image::ImageBuffer::new(size, size);
        let resized_rgba = resized.to_rgba8();
        for y in 0..content_size {
            for x in 0..content_size {
                canvas.put_pixel(x + padding, y + padding, *resized_rgba.get_pixel(x, y));
            }
        }
        
        let image = ico::IconImage::from_rgba_data(size, size, canvas.into_raw());
        icon_dir.add_entry(ico::IconDirEntry::encode(&image).map_err(|e| AppError::Internal(e.to_string()))?);
    }

    let file = File::create(output_dir.join("icon.ico"))
        .map_err(|e| AppError::Internal(format!("Failed to create ICO: {}", e)))?;
    let mut writer = BufWriter::new(file);
    icon_dir.write(&mut writer).map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(())
}
