mod adapters;
mod event;
mod notification;
mod server;
mod task_manager;

use task_manager::TaskManager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let task_manager = TaskManager::default();

    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .manage(task_manager.clone())
        .setup(move |app| {
            server::spawn(app.handle().clone(), task_manager);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
