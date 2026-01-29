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
                let s = String::from_utf8_lossy(&output.stdout).trim().to_string();
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
    
    let adb_sidecar_version = if let Ok(cmd) = adb_sidecar_cmd {
        let output = cmd.args(&["--version"]).output().await.ok();
        output.and_then(|out| {
             if out.status.success() {
                String::from_utf8_lossy(&out.stdout).lines().next().map(|line| line.to_string())
            } else {
                None
            }
        })
    } else {
        None
    };

    let adb = if adb_sidecar_version.is_some() {
        adb_sidecar_version
    } else {
        get_version("adb", &["--version"])
    };







    // Android Studio
    let studio_paths = [
        "C:\\Program Files\\Android\\Android Studio\\bin\\studio64.exe",
        "C:\\Program Files\\Android\\Android Studio\\bin\\studio.exe",
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
            version: xcode,
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


