use crate::errors::AppResult;
use tauri::{AppHandle, Manager, Emitter};
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
        let (prog, final_args) = ("cmd", [&["/C", &command], &args[..]].concat());
        
        #[cfg(not(target_os = "windows"))]
        let (prog, final_args) = (command.as_str(), args);

        let mut cmd = Command::new(prog);
        #[cfg(target_os = "windows")]
        cmd.args(final_args);
        #[cfg(not(target_os = "windows"))]
        cmd.args(final_args);
        
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
