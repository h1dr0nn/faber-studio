use std::fs;
use tauri::{AppHandle, Manager};
use serde_json::Value;

#[tauri::command]
pub async fn save_config(app: AppHandle, key: String, value: Value) -> Result<(), String> {
    let app_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    
    if !app_dir.exists() {
        fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;
    }
    
    let config_path = app_dir.join(format!("{}.json", key));
    let content = serde_json::to_string_pretty(&value).map_err(|e| e.to_string())?;
    
    fs::write(config_path, content).map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
pub async fn load_config(app: AppHandle, key: String) -> Result<Value, String> {
    let app_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let config_path = app_dir.join(format!("{}.json", key));
    
    if !config_path.exists() {
        return Ok(Value::Null);
    }
    
    let content = fs::read_to_string(config_path).map_err(|e| e.to_string())?;
    let value: Value = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    
    Ok(value)
}
