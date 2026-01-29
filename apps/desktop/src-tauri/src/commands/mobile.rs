use crate::errors::AppResult;
use serde::Serialize;


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

    // Android Devices via ADB Sidecar
    // app.shell().sidecar("adb") returns Result<Command, Error>
    let sidecar = app.shell().sidecar("adb")
        .map_err(|e| crate::errors::AppError::Command(format!("Failed to create sidecar: {}", e)))?;
        
    let output = sidecar
        .args(&["devices", "-l"])
        .output()
        .await
        .map_err(|e| crate::errors::AppError::Command(e.to_string()))?;


        
    if output.status.code() == Some(0) {
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
