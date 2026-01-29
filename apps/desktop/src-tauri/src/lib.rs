pub mod commands;
pub mod errors;
pub mod models;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::env_check::check_environment,
            commands::project_init::scan_project,
            commands::project_init::read_file,
            commands::project_init::write_file,
            commands::runner::run_command,
            commands::assets::get_assets,
            commands::assets::generate_icons,
            commands::assets::find_icons_dir,
            commands::mobile::get_devices,
            commands::search::search_in_files,
            commands::git::git_status,
            commands::git::git_stage,
            commands::git::git_unstage,
            commands::git::git_commit,
            commands::git::git_diff_staged,
            commands::git::git_push,
            commands::git::git_pull,
            commands::git::git_discard_changes,
            commands::git::git_branch,
            commands::config::save_config,
            commands::config::load_config,
        ])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
