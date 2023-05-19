// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn read_text(path: &str) -> String {
    format!("read_text {}", path);
    let contents = fs::read_to_string(path).expect("无法读取文件");
    println!("文件打开。{}", path);
    return contents;
}

#[tauri::command]
fn watch_dir(path: &str) {
    fn event_fn(res: notify::Result<notify::Event>) {
        match res {
            Ok(event) => println!("event: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
    let mut watcher1 = notify::recommended_watcher(event_fn);
}

// 选择目录
#[tauri::command]
fn on_dir() -> Option<PathBuf> {
    let file_path = tauri::api::dialog::blocking::FileDialogBuilder::new().pick_folder();
    return file_path;
}

#[tauri::command]
fn save_text(path: &str, content: &str) -> bool {
    let mut file = fs::File::create(path).expect("无法创建文件");
    file.write_all(content.as_bytes()).expect("无法写入文件");
    println!("文件写入完成。");
    return true;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet, read_text, save_text, on_dir
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
