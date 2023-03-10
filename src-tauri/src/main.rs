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
fn open_file_in_explorer(path: &str) {
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
        let data = "{\n    \"toolListConfig\":[\n        {\n            \"name\":\"合并固件\",\n            \"value\":\"mergeBin\",\n            \"toast\":null,\n            \"dropDesc\":\"选择或者拖拽build目录到此\",\n            \"dropHelp\":\"请在执行idf.py build后再使用\",\n            \"dropRegex\":\"(build)$\",\n            \"isDirectory\":true,\n            \"cmd\":[\n                \"--chip\",\n                \"${chip}\",\n                \"merge_bin\",\n                \"-o\",\n                \"${appName}\",\n                \"${flashArgs}\"\n            ]\n        },\n        {\n            \"name\":\"烧录\",\n            \"value\":\"flash\",\n            \"toast\":\"烧录未合并的固件\",\n            \"dropDesc\":\"选择或者拖拽build目录到此\",\n            \"dropHelp\":\"请在执行idf.py build后再使用\",\n            \"dropRegex\":\"(build)$\",\n            \"isDirectory\":true,\n            \"cmd\":[\n                \"--chip\",\n                \"${chip}\",\n                \"-p\",\n                \"${port}\",\n                \"-b\",\n                \"1152000\",\n                \"--before=default_reset\",\n                \"--after=hard_reset\",\n                \"write_flash\",\n                \"${flashArgs}\"\n            ]\n        },\n        {\n            \"name\":\"烧录单个\",\n            \"value\":\"flashSingle\",\n            \"toast\":\"烧录合并后的固件\",\n            \"dropDesc\":\"选择或者拖拽bin文件到此\",\n            \"dropHelp\":\"只支持下载地址为0x0的固件\",\n            \"dropRegex\":\".(bin)$\",\n            \"isDirectory\":false,\n            \"cmd\":[\n                \"-p\",\n                \"${port}\",\n                \"-b\",\n                \"1152000\",\n                \"write_flash\",\n                \"0x0\",\n                \"${path}\"\n            ]\n        }\n    ],\n    \"customToolList\":[\n        {\n            \"name\":\"擦除固件\",\n            \"toast\":\"擦除固件\",\n            \"cmd\":[\n                \"-p\",\n                \"${port}\",\n                \"erase_flash\"\n            ]\n        },\n        {\n            \"name\":\"获取flash大小\",\n            \"toast\":\"获取flash大小\",\n            \"cmd\":[\n                \"-p\",\n                \"${port}\",\n                \"flash_id\"\n            ]\n        }\n    ]\n}";
        fs::write("tools.config.json", data).unwrap();
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            get_serial_port_list,
            get_current_dir,
            open_file_in_explorer
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
