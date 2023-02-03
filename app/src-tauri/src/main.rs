#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(payment: &str) -> String {
    format!("New payment: {}!", payment)
}

#[tauri::command]
fn expense(expense: &str) -> String {
    format!("New expense: {}!", expense)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, expense])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
