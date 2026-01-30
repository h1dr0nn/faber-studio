use crate::errors::AppResult;
use tauri::{AppHandle, Emitter};
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use std::thread;

#[tauri::command]
pub async fn run_command(app: AppHandle, command: String, args: Vec<String>, cwd: Option<String>) -> AppResult<String> {
    let task_id = uuid::Uuid::new_v4().to_string();
    let task_id_clone = task_id.clone();
    let app_handle = app.clone();

    thread::spawn(move || {
        // Prepare command
        // On Windows, shelling out to cmd /C is often safer for "npm", "pnpm" which are batches
        #[cfg(target_os = "windows")]
        let (prog, final_args) = {
            let mut a = vec!["/C".to_string(), command];
            a.extend(args);
            ("cmd", a)
        };
        
        #[cfg(not(target_os = "windows"))]
        let (prog, final_args) = (command, args);

        let mut cmd = Command::new(prog);
        cmd.args(&final_args);
        
        // Inherit environment variables from parent process
        // Filter out pnpm/npm-specific config vars that conflict
        let mut filtered_env: std::collections::HashMap<String, String> = std::env::vars()
            .filter(|(key, _)| {
                let key_lower = key.to_lowercase();
                // Filter npm config vars (npm_config_xxx)
                !key_lower.starts_with("npm_config_") && 
                !key_lower.starts_with("npm_package_") &&
                !key_lower.starts_with("npm_lifecycle_") &&
                // Filter pnpm vars
                !key_lower.starts_with("pnpm_") &&
                // Filter specific problematic vars
                key_lower != "init_cwd" &&
                key_lower != "npm_execpath"
            })
            .collect();
        
        // Add NODE_PATH to help npm find modules
        if let Some(ref dir) = cwd {
            let node_modules = std::path::Path::new(dir).join("node_modules");
            if node_modules.exists() {
                filtered_env.insert("NODE_PATH".to_string(), node_modules.to_string_lossy().to_string());
            }
        }
        
        cmd.envs(filtered_env);

        
        // Hide window on Windows to avoid popping up console windows
        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x08000000;
            cmd.creation_flags(CREATE_NO_WINDOW);
        }

        if let Some(path) = cwd {
            cmd.current_dir(path);
        }

        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        match cmd.spawn() {
            Ok(mut child) => {
                 let _ = app_handle.emit("runner-start", &task_id_clone);

                // Handle stdout
                if let Some(stdout) = child.stdout.take() {
                    let app_h = app_handle.clone();
                    let t_id = task_id_clone.clone();
                    thread::spawn(move || {
                        let reader = BufReader::new(stdout);
                        for line in reader.lines() {
                            if let Ok(l) = line {
                                let _ = app_h.emit("runner-stdout", serde_json::json!({ "taskId": t_id, "message": l }));
                            }
                        }
                    });
                }

                // Handle stderr
                if let Some(stderr) = child.stderr.take() {
                    let app_h = app_handle.clone();
                    let t_id = task_id_clone.clone();
                    thread::spawn(move || {
                        let reader = BufReader::new(stderr);
                        for line in reader.lines() {
                            if let Ok(l) = line {
                                let _ = app_h.emit("runner-stderr", serde_json::json!({ "taskId": t_id, "message": l }));
                            }
                        }
                    });
                }

                // Wait for exit
                match child.wait() {
                    Ok(status) => {
                        let _ = app_handle.emit("runner-exit", serde_json::json!({ "taskId": task_id_clone, "code": status.code() }));
                    }
                    Err(e) => {
                         let _ = app_handle.emit("runner-error", serde_json::json!({ "taskId": task_id_clone, "error": e.to_string() }));
                    }
                }
            }
            Err(e) => {
                let _ = app_handle.emit("runner-error", serde_json::json!({ "taskId": task_id_clone, "error": e.to_string() }));
            }
        }
    });

    Ok(task_id)
}
