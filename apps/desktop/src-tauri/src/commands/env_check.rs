use crate::models::CheckResult;
use crate::errors::AppResult;
use std::process::Command;

fn get_version(cmd: &str, args: &[&str]) -> Option<String> {
    Command::new(cmd)
        .args(args)
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
                
                let s = if !stdout.is_empty() { stdout } else { stderr };
                
                // Simple version extractor (usually it's the first line or has 'version' in it)
                s.lines().next().map(|line| line.to_string())
            } else {
                None
            }
        })
}

#[tauri::command]
pub async fn check_environment(app: tauri::AppHandle) -> AppResult<Vec<CheckResult>> {
    let mut results = Vec::new();

    // Node.js
    let node = get_version("node", &["--version"]);
    results.push(CheckResult {
        name: "Node.js".into(),
        status: node.is_some(),
        version: node,
        message: None,
    });

    // Git
    let git = get_version("git", &["--version"]);
    results.push(CheckResult {
        name: "Git".into(),
        status: git.is_some(),
        version: git,
        message: None,
    });

    // Rust
    let rust = get_version("rustc", &["--version"]);
    results.push(CheckResult {
        name: "Rust".into(),
        status: rust.is_some(),
        version: rust,
        message: None,
    });

    // Cargo
    let cargo = get_version("cargo", &["--version"]);
    results.push(CheckResult {
        name: "Cargo".into(),
        status: cargo.is_some(),
        version: cargo,
        message: None,
    });

    // ADB (for Android)
    use tauri_plugin_shell::ShellExt;

    // Try sidecar first (configured as "adb" in tauri.conf.json)
    let adb_sidecar_cmd = app.shell().sidecar("adb");
    let sidecar_exists = adb_sidecar_cmd.is_ok();
    println!("DEBUG: adb_sidecar_cmd result is ok: {}", sidecar_exists);
    
    let adb_sidecar_version = if let Ok(cmd) = adb_sidecar_cmd {
        match cmd.args(&["--version"]).output().await {
            Ok(out) => {
                println!("DEBUG: adb sidecar executed successfully");
                if out.status.success() {
                    let v = String::from_utf8_lossy(&out.stdout).lines().next().map(|line| line.to_string());
                    println!("DEBUG: adb sidecar version: {:?}", v);
                    v
                } else {
                    println!("DEBUG: adb sidecar status failed: {:?}", out.status);
                    None
                }
            },
            Err(e) => {
                println!("DEBUG: adb sidecar execution error: {:?}", e);
                None
            }
        }
    } else {
        println!("DEBUG: adb sidecar not found in config or missing binary");
        None
    };

    // Determine ADB status
    let sidecar_ran_ok = adb_sidecar_version.is_some();
    
    let (adb_name, adb_status, adb_version, adb_message) = if sidecar_ran_ok {
        // Sidecar found and ran successfully
        ("ADB (Internal Bundle)".to_string(), true, adb_sidecar_version, None)
    } else if sidecar_exists {
        // Sidecar found but failed to run (likely DLL issue in dev mode)
        ("ADB (Internal Bundle)".to_string(), true, Some("Bundled (dev mode - DLL issue)".to_string()), 
         Some("Binary bundled. Will work in production build.".to_string()))
    } else {
        // Check system ADB
        let v = get_version("adb", &["--version"]);
        println!("DEBUG: adb system version: {:?}", v);
        if v.is_some() {
            ("ADB (System)".to_string(), true, v, None)
        } else {
            ("ADB".to_string(), false, None, Some("ADB not found".to_string()))
        }
    };

    results.push(CheckResult {
        name: adb_name,
        status: adb_status,
        version: adb_version,
        message: adb_message, 
    });

    // Java (Required for Android development)
    let java = get_version("java", &["-version"]);
    results.push(CheckResult {
        name: "Java (JDK)".into(),
        status: java.is_some(),
        version: java.clone(),
        message: if java.is_none() { Some("Install JDK 17+ for Android builds".into()) } else { None },
    });

    // Android Studio
    let studio_paths = [
        // Windows
        "C:\\Program Files\\Android\\Android Studio\\bin\\studio64.exe",
        "C:\\Program Files\\Android\\Android Studio\\bin\\studio.exe",
        // macOS
        "/Applications/Android Studio.app",
        "/Applications/Android Studio Preview.app",
        // Linux
        "/opt/android-studio/bin/studio.sh",
        "/usr/local/android-studio/bin/studio.sh",
    ];
    let studio_found = studio_paths.iter().any(|p| std::path::Path::new(p).exists());
    results.push(CheckResult {
        name: "Android Studio".into(),
        status: studio_found,
        version: if studio_found { Some("Installed".into()) } else { None },
        message: if !studio_found { Some("Install Android Studio for mobile development".into()) } else { None },
    });

    // Python
    let python = get_version("python", &["--version"]);
    results.push(CheckResult {
        name: "Python".into(),
        status: python.is_some(),
        version: python,
        message: None,
    });

    // Xcode (macOS only)

    #[cfg(target_os = "macos")]
    {
        let xcode = get_version("xcodebuild", &["-version"]);
        results.push(CheckResult {
            name: "Xcode".into(),
            status: xcode.is_some(),
            version: xcode.clone(),
            message: if xcode.is_none() { Some("Required for iOS development".into()) } else { None },
        });
    }
    #[cfg(not(target_os = "macos"))]
    {
        results.push(CheckResult {
            name: "Xcode".into(),
            status: false,
            version: None,
            message: Some("macOS required for iOS development".into()),
        });
    }

    Ok(results)
}


