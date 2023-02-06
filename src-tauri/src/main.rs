#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serialport::available_ports;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_serial_port_list() -> String {
    let port_info_list = available_ports().unwrap();
    let port_list: Vec<String> = port_info_list
        .iter()
        .map(|x| x.port_name.to_string())
        .collect();
    let json_data = serde_json::to_string(&port_list).unwrap();
    return json_data;
}

#[tauri::command]
fn get_current_dir() -> String {
    let path = env::current_dir().unwrap();
    return path.display().to_string();
}

#[tauri::command]
fn open_dir_in_explorer(path: &str) {
    let platform = env::consts::OS.to_string();
    let mut program = "explorer";
    if platform == "windows".to_string() {
        program = "explorer";
    }
    if platform == "macos".to_string() {
        program = "open";
    }
    if platform == "linux".to_string() {
        program = "xdg-open";
    }
    Command::new(program).arg(path).spawn().unwrap();
}

fn main() {
    if !Path::new("firmware").exists() {
        fs::create_dir("firmware").unwrap();
    }

    if !Path::new("esptool").exists() {
        fs::create_dir("esptool").unwrap();
    }

    if !Path::new("tools.config.json").exists() {
        let data = "{\n\t\"toolListConfig\": [\n\t\t{\n\t\t\t\"name\": \"合并固件\",\n\t\t\t\"value\": \"mergeBin\",\n\t\t\t\"toast\": null,\n\t\t\t\"dropDesc\": \"选择或者拖拽build目录到此\",\n\t\t\t\"dropHelp\": \"请在执行idf.py build后再使用\",\n\t\t\t\"dropRegex\": \"(build)$\",\n\t\t\t\"cmd\": [\n\t\t\t\t\"--chip\",\n\t\t\t\t\"${chip}\",\n\t\t\t\t\"merge_bin\",\n\t\t\t\t\"-o\",\n\t\t\t\t\"${appName}\",\n\t\t\t\t\"${flashArgs}\"\n\t\t\t]\n\t\t},\n\t\t{\n\t\t\t\"name\": \"烧录\",\n\t\t\t\"value\": \"flash\",\n\t\t\t\"toast\": \"烧录未合并的固件\",\n\t\t\t\"dropDesc\": \"选择或者拖拽build目录到此\",\n\t\t\t\"dropHelp\": \"请在执行idf.py build后再使用\",\n\t\t\t\"dropRegex\": \"(build)$\",\n\t\t\t\"cmd\": [\n\t\t\t\t\"--chip\",\n\t\t\t\t\"${chip}\",\n\t\t\t\t\"-p\",\n\t\t\t\t\"${port}\",\n\t\t\t\t\"-b\",\n\t\t\t\t\"1152000\",\n\t\t\t\t\"--before=default_reset\",\n\t\t\t\t\"--after=hard_reset\",\n\t\t\t\t\"write_flash\",\n\t\t\t\t\"${flashArgs}\"\n\t\t\t]\n\t\t},\n\t\t{\n\t\t\t\"name\": \"烧录单个\",\n\t\t\t\"value\": \"flashSingle\",\n\t\t\t\"toast\": \"烧录合并后的固件\",\n\t\t\t\"dropDesc\": \"选择或者拖拽bin文件到此\",\n\t\t\t\"dropHelp\": \"只支持下载地址为0x0的固件\",\n\t\t\t\"dropRegex\": \".(bin)$\",\n\t\t\t\"cmd\": [\n\t\t\t\t\"-p\",\n\t\t\t\t\"${port}\",\n\t\t\t\t\"-b\",\n\t\t\t\t\"1152000\",\n\t\t\t\t\"write_flash\",\n\t\t\t\t\"0x0\",\n\t\t\t\t\"${path}\"\n\t\t\t]\n\t\t}\n\t],\n\t\"customToolList\": [\n\t\t{\n\t\t\t\"name\": \"擦除固件\",\n\t\t\t\"toast\": \"擦除固件\",\n\t\t\t\"cmd\": [\n\t\t\t\t\"-p\",\n\t\t\t\t\"${port}\",\n\t\t\t\t\"erase_flash\"\n\t\t\t]\n\t\t},\n\t\t{\n\t\t\t\"name\": \"获取flash大小\",\n\t\t\t\"toast\": \"获取flash大小\",\n\t\t\t\"cmd\": [\n\t\t\t\t\"-p\",\n\t\t\t\t\"${port}\",\n\t\t\t\t\"flash_id\"\n\t\t\t]\n\t\t}\n\t]\n}";
        fs::write("tools.config.json", data).unwrap();
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            get_serial_port_list,
            get_current_dir,
            open_dir_in_explorer
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
