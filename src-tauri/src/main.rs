// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use hotwatch::{Event, Hotwatch};
use serde::Serialize;
use std::io::prelude::*;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::{fs, thread};
use tauri::{window, Manager};

struct AppState {
    stop_flag: bool,
    thread_handle: Option<thread::JoinHandle<()>>,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

//读取文件信息
#[tauri::command]
fn read_text(path: &str) -> String {
    format!("read_text {}", path);
    let contents = fs::read_to_string(path).expect("无法读取文件");
    println!("文件打开。{}", path);
    return contents;
}

//创建文件
#[tauri::command]
fn create_file(path: &str) -> bool {
    println!("create_file {}", path);
    // 创建文件
    let status = match fs::File::create(path) {
        Ok(_) => true,
        Err(err) => {
            println!("无法创建文件: {}", err);
            return false;
        }
    };
    return status;
}

#[derive(Serialize)]
struct FileEvent {
    path: String,
    event_type: String,
}

//监听文件夹
#[tauri::command]
fn watch_dir(path: &str, window: tauri::Window, state: tauri::State<Arc<Mutex<AppState>>>) {
    println!("watch_dir {}", path);
    let mut app_state = state.lock().unwrap();
    if app_state.thread_handle.is_some() {
        app_state.stop_flag = true;
    }
    let dir = path.to_string();
    let hotwatch = Arc::new(Mutex::new(
        Hotwatch::new().expect("hotwatch failed to initialize!"),
    ));
    let stop_flag = Arc::new(Mutex::new(app_state.stop_flag));
    let hotwatch_thread = thread::spawn(move || {
        hotwatch
            .lock()
            .unwrap()
            .watch(dir, move |event: Event| {
                let file_event = match event {
                    Event::Create(path) => FileEvent {
                        path: path.to_string_lossy().into_owned(),
                        event_type: "create".to_string(),
                    },
                    Event::Write(path) => FileEvent {
                        path: path.to_string_lossy().into_owned(),
                        event_type: "write".to_string(),
                    },
                    Event::Remove(path) => FileEvent {
                        path: path.to_string_lossy().into_owned(),
                        event_type: "remove".to_string(),
                    },
                    Event::Rename(old_path, new_path) => FileEvent {
                        path: format!(
                            "{} -> {}",
                            old_path.to_string_lossy(),
                            new_path.to_string_lossy()
                        ),
                        event_type: "rename".to_string(),
                    },
                    _ => return,
                };
                let json_data = serde_json::to_string(&file_event).unwrap();
                println!("{}", json_data);
                window
                    .emit("file_event", Some(json_data))
                    .expect("发送失败")
            })
            .expect("failed to watch file!");
        loop {
            if *stop_flag.lock().unwrap() {
                println!("remove_watch_dir");
                break;
            }
        }
    });
    app_state.thread_handle = Some(hotwatch_thread);
    app_state.stop_flag = false;
}

#[tauri::command]
fn remove_watch_dir(state: tauri::State<Arc<Mutex<AppState>>>) {
    let mut app_state = state.lock().unwrap();
    app_state.stop_flag = true;
}

#[derive(Serialize)]
struct FileEntry {
    name: String,
    path: String,
}

//获取目录文件信息
fn get_files_in_directory(dir_path: &str) -> Result<Vec<FileEntry>, std::io::Error> {
    let mut file_entries = Vec::new();
    let dir = fs::read_dir(dir_path)?;

    for entry in dir {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let file_name = path.file_name().unwrap().to_string_lossy().into_owned();
            let file_path = path.to_string_lossy().into_owned();
            let file_entry = FileEntry {
                name: file_name,
                path: file_path,
            };
            file_entries.push(file_entry);
        }
    }
    Ok(file_entries)
}

//读取目录
#[tauri::command]
fn read_dir(path: &str) -> String {
    let file_entries = get_files_in_directory(path).unwrap();
    let json_data = serde_json::to_string(&file_entries).unwrap();
    return json_data;
}

// 选择目录
#[tauri::command]
fn on_dir() -> Option<PathBuf> {
    let file_path = tauri::api::dialog::blocking::FileDialogBuilder::new().pick_folder();
    return file_path;
}

//保存文件
#[tauri::command]
fn save_text(path: &str, content: &str) -> bool {
    let mut file = fs::File::create(path).expect("无法创建文件");
    file.write_all(content.as_bytes()).expect("无法写入文件");
    println!("文件写入完成。");
    return true;
}

//使用浏览器打开链接
#[tauri::command]
fn open_link(href: &str) -> bool {
    if webbrowser::open(href).is_ok() {
        return true;
    }
    return false;
}

#[tauri::command]
fn register_network_request_interceptor(window: tauri::Window) {
    println!("register_network_request_interceptor");
}

fn main() {
    let app_state = Arc::new(Mutex::new(AppState {
        stop_flag: false,
        thread_handle: None,
    }));
    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            greet,
            read_text,
            save_text,
            on_dir,
            watch_dir,
            remove_watch_dir,
            read_dir,
            create_file,
            open_link,
            register_network_request_interceptor,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
