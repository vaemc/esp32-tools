#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serialport::{available_ports};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_serial_port_list() -> String {
    match available_ports() {
        Ok(ports) => {
            let mut port_list  = Vec::new();
            for p in ports {
                port_list.push(p.port_name);
            }
            let json_data = serde_json::to_string(&port_list).unwrap();
            return json_data;
        }
        Err(_e) => {
            return "[]".to_string();
        }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,get_serial_port_list])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
