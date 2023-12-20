use chrono::DateTime;
use chrono::Local;
use tauri::Manager;
use tauri::command;
use std::{env, fs, path::{Path, PathBuf}, process::Command};

use super::responses::*;

// close splashscreen
#[command]
pub async fn close_splashscreen(window: tauri::Window) {
    check_paradox_game();
    if let Some(splashscreen) = window.get_window("splashscreen") {
        splashscreen.close().unwrap();
    }
    // show main window
    window.get_window("main").unwrap().show().unwrap();
}

#[command(rename_all = "snake_case")]
pub fn load_savefiles(game_name: String) -> Result<Vec<FormItem>, String> {
    let mut form_items = vec![];

    let user_address = match env::var("USERPROFILE") {
        Ok(val) => val,
        Err(_e) => panic!("Unable to get address!"),
    };
    let mut path = PathBuf::from(&user_address);
    path.push("Documents");
    path.push("Paradox Interactive");
    match &game_name[..] {
        "eu4" => {
            path.push("Europa Universalis IV");
        },
        "ck3" => {
            path.push("Crusader Kings III");
        },
        "hoi4" => {
            path.push("Hearts of Iron IV");
        },
        "stellaris" => {
            path.push("Stellaris");
        },
        "v3" => {
            path.push("Victoria 3");
        },
        "ck2" => {
            path.push("Crusader Kings II");
        },
        _ => panic!("Don't have this game!"),
    }
    path.push("save games");
    visit_files_and_folders(&path, &mut form_items, false);
    form_items.sort_by(|a, b| b.update_time.cmp(&a.update_time));
    Ok(form_items)
}

#[command(rename_all = "snake_case")]
pub fn open_file_explorer(game_name: String) {
    let user_address = match env::var("USERPROFILE") {
        Ok(val) => val,
        Err(_e) => panic!("Unable to get address!"),
    };
    let mut path = PathBuf::from(&user_address);
    path.push("Documents");
    path.push("Paradox Interactive");
    match &game_name[..] {
        "eu4" => {
            path.push("Europa Universalis IV");
        },
        "ck3" => {
            path.push("Crusader Kings III");
        },
        "hoi4" => {
            path.push("Hearts of Iron IV");
        },
        "v3" => {
            path.push("Victoria 3");
        },
        "stellaris" => {
            path.push("Stellaris");
        },
        "ck2" => {
            path.push("Crusader Kings II");
        },
        _ => panic!("Don't have this game!"),
    }
    path.push("save games");
    Command::new("explorer")
        .arg(path)
        .spawn()
        .expect("failed to open file explorer");
}

fn check_paradox_game() {
    let user_address = match env::var("USERPROFILE") {
        Ok(val) => val,
        Err(_e) => panic!("Unable to get address!"),
    };
    let mut path = PathBuf::from(&user_address);
    path.push("Documents");
    path.push("Paradox Interactive");
    let paths = fs::read_dir(path).unwrap();

    for path in paths {
        if let Ok(game_path) = path {
            let mut backups_path = game_path.path();
            backups_path.push("save games");
            backups_path.push("Backups");
            if !Path::new(&backups_path).exists() {
                let _ = fs::create_dir(backups_path);
            }
        }
    }
}

fn visit_files_and_folders(path: &PathBuf, result: &mut Vec<FormItem>, backups: bool) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let entry_path = entry.path();
                if entry_path.is_file() {
                    let time = DateTime::<Local>::from(fs::metadata(&entry_path).unwrap().modified().unwrap());
                    let form_item = FormItem {
                        name: entry_path.file_name().unwrap().to_string_lossy().to_string(),
                        location: entry_path.as_path().display().to_string(),
                        update_time: time.format("%Y/%m/%d %H:%M:%S").to_string(),
                        is_backups: backups,
                    };
                    result.push(form_item);
                }
                else if entry_path.is_dir() {
                    let filename = entry_path.file_name().unwrap().to_string_lossy().to_string();
                    visit_files_and_folders(&entry_path, result, filename == "Backups".to_string());
                }
            }
        }
    }
}