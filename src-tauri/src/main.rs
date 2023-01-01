#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem,
};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .system_tray(create_system_tray())
        .on_system_tray_event(|app, event| system_tray_events(app, event))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// Creates the system tray menu.
fn create_system_tray() -> SystemTray {
    let quit = CustomMenuItem::new("Quit".to_string(), "Quit");
    let show = CustomMenuItem::new("Show".to_string(), "Show");
    let hide = CustomMenuItem::new("Hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_item(show)
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);
    SystemTray::new().with_menu(tray_menu)
}

/// Handle system tray events
fn system_tray_events(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "Hide" => {
                let window = app.get_window("main").unwrap();
                window.hide().unwrap();
            }
            "Show" => {
                let window = app.get_window("main").unwrap();
                window.show().unwrap();
            }
            "Quit" => {
                std::process::exit(0);
            }
            _ => {}
        },
        _ => {}
    }
}
