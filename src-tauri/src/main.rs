// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod responses;
mod load;
mod operate;
mod listen;
use load::{close_splashscreen, load_savefiles, open_file_explorer};
use operate::{replace_savefile, delete_savefile, back_up_savefile};
use listen::listen_savefiles;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            close_splashscreen,
            load_savefiles,
            replace_savefile,
            delete_savefile,
            listen_savefiles,
            back_up_savefile,
            open_file_explorer,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
