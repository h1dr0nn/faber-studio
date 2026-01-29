use crate::errors::AppResult;
use serde::Serialize;
use std::process::Command;
use std::path::PathBuf;
use tauri::Manager;


#[derive(Serialize, Clone, Debug)]
pub struct MobileDevice {
    pub id: String,
    pub model: String,
    pub manufacturer: String,
    pub platform: String, // "android" | "ios"
}

#[tauri::command]
pub async fn get_devices(app: tauri::AppHandle) -> AppResult<Vec<MobileDevice>> {
    let mut devices = Vec::new();

    // Get the path to the binaries folder
    let resource_dir = app.path().resource_dir()
        .map_err(|e| crate::errors::AppError::Command(format!("Failed to get resource dir: {}", e)))?;
    
    // In dev mode, the binaries are in src-tauri/binaries
    // In production, they're in the resources folder
    let mut adb_path = resource_dir.join("binaries").join("adb-x86_64-pc-windows-msvc.exe");
    
    if !adb_path.exists() {
        // Try dev mode path
        let exe_dir = std::env::current_exe()
            .map_err(|e| crate::errors::AppError::Command(format!("Failed to get exe path: {}", e)))?;
        let dev_path = exe_dir.parent()
            .and_then(|p| p.parent())
            .and_then(|p| p.parent())
            .map(|p| p.join("binaries").join("adb-x86_64-pc-windows-msvc.exe"));
        
        if let Some(path) = dev_path {
            if path.exists() {
                adb_path = path;
            }
        }
    }
    
    // Last resort: try the src-tauri/binaries directly
    if !adb_path.exists() {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        adb_path = manifest_dir.join("binaries").join("adb-x86_64-pc-windows-msvc.exe");
    }
    
    log::info!("ADB path: {:?}", adb_path);
    
    if !adb_path.exists() {
        log::error!("ADB executable not found at {:?}", adb_path);
        return Err(crate::errors::AppError::Command("ADB not found".into()));
    }
    
    // Get the directory containing ADB (where DLLs should be)
    let adb_dir = adb_path.parent().unwrap();
    log::info!("ADB directory (for DLLs): {:?}", adb_dir);
    
    // Run ADB with the binaries folder as working directory so DLLs are found
    let output = Command::new(&adb_path)
        .current_dir(adb_dir)
        .args(&["devices", "-l"])
        .output()
        .map_err(|e| {
            log::error!("ADB execution failed: {}", e);
            crate::errors::AppError::Command(e.to_string())
        })?;

    log::info!("ADB exit code: {:?}", output.status.code());
    log::info!("ADB stdout: {}", String::from_utf8_lossy(&output.stdout));
    log::info!("ADB stderr: {}", String::from_utf8_lossy(&output.stderr));

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        // Parse "List of devices attached"
        // HT75B0200000 device product:marlin model:Pixel_XL device:marlin transport_id:1
        
        for line in stdout.lines().skip(1) {
            if line.trim().is_empty() { continue; }
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 2 { continue; }
            
            let id = parts[0].to_string();
            if id.contains("List") { continue; } // Header sometimes not skipped if empty line missing
            
            let mut model = "Unknown".to_string();
            let mut product = "Unknown".to_string();
            
            for part in &parts {
                if part.starts_with("model:") {
                    model = part.replace("model:", "");
                }
                if part.starts_with("product:") {
                    product = part.replace("product:", "");
                }
            }
            
            devices.push(MobileDevice {
                id,
                model,
                manufacturer: product, 
                platform: "android".to_string(),
            });
        }
    }

    Ok(devices)
}
