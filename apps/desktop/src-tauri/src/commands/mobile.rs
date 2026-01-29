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
    use tauri_plugin_shell::ShellExt;
    let mut devices = Vec::new();

    log::info!("Fetching devices using adb sidecar");

    let output = app
        .shell()
        .sidecar("adb")
        .map_err(|e| {
            log::error!("Failed to create adb sidecar: {}", e);
            crate::errors::AppError::Command(format!("ADB sidecar not found: {}", e))
        })?
        .args(&["devices", "-l"])
        .output()
        .await
        .map_err(|e| {
            log::error!("ADB execution failed: {}", e);
            crate::errors::AppError::Command(format!("ADB execution failed: {}", e))
        })?;

    log::info!("ADB exit code: {:?}", output.status.code());
    log::info!("ADB stdout: {}", String::from_utf8_lossy(&output.stdout));
    log::info!("ADB stderr: {}", String::from_utf8_lossy(&output.stderr));

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        // Parse "List of devices attached"
        
        for line in stdout.lines().skip(1) {
            if line.trim().is_empty() { continue; }
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 2 { continue; }
            
            let id = parts[0].to_string();
            if id.contains("List") { continue; }
            
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
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(crate::errors::AppError::Command(format!("ADB error: {}", stderr)));
    }

    Ok(devices)
}
