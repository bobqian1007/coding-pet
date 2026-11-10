use tauri::AppHandle;
use tauri_plugin_notification::NotificationExt;

pub fn show_completion(app: &AppHandle) {
    if let Err(error) = app
        .notification()
        .builder()
        .title("Coding Pet")
        .body("Codex 单轮执行已完成")
        .show()
    {
        eprintln!("failed to show completion notification: {error}");
    }
}
