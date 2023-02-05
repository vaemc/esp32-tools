#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serialport::available_ports;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

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

fn main() {

    
    // let config_json=b"{\"toolListConfig\":[{\"name\":\"合并固件\",\"value\":\"mergeBin\",\"toast\":null,\"dropDesc\":\"选择或者拖拽build目录到此\",\"dropHelp\":\"请在执行idf.pybuild后再使用\",\"dropRegex\":\"(build)$\",\"cmd\":[\"--chip\",\"${chip}\",\"merge_bin\",\"-o\",\"${appName}\",\"${flashArgs}\"]},{\"name\":\"烧录\",\"value\":\"flash\",\"toast\":\"烧录未合并的固件\",\"dropDesc\":\"选择或者拖拽build目录到此\",\"dropHelp\":\"请在执行idf.pybuild后再使用\",\"dropRegex\":\"(build)$\",\"cmd\":[\"--chip\",\"${chip}\",\"-p\",\"${port}\",\"-b\",\"1152000\",\"--before=default_reset\",\"--after=hard_reset\",\"write_flash\",\"${flashArgs}\"]},{\"name\":\"烧录单个\",\"value\":\"flashSingle\",\"toast\":\"烧录合并后的固件\",\"dropDesc\":\"选择或者拖拽bin文件到此\",\"dropHelp\":\"只支持下载地址为0x0的固件\",\"dropRegex\":\".(bin)$\",\"cmd\":[\"-p\",\"${port}\",\"write_flash\",\"0x0\",\"${path}\"]}],\"customToolList\":[{\"name\":\"擦除固件\",\"toast\":\"擦除固件\",\"cmd\":[\"-p\",\"${port}\",\"erase_flash\"]},{\"name\":\"获取flash大小\",\"toast\":\"获取flash大小\",\"cmd\":[\"-p\",\"${port}\",\"flash_id\"]}]}";
   
    // if !Path::new("firmware").exists() {
    //     fs::create_dir("firmware").unwrap();
    // }


    if !Path::new("config.json").exists() {
        File::create("config.json")
            .unwrap()
            .write_all(b"config_json")
            .unwrap();
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            get_serial_port_list,
            get_current_dir
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
