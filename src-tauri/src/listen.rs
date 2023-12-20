use tauri::command;
use serde::Deserialize;
use std::path::PathBuf;
use chrono::{DateTime, Local};
use std::{fs::{metadata, File}, cmp::Ordering::*, io::{Write, Read}};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListenParam {
    location: String,
    update_time: String,
}

#[command(rename_all = "snake_case")]
pub fn listen_savefiles(savefiles: Vec<ListenParam>) {
    let mut path;
    for savefile in savefiles {
        path = PathBuf::from(&savefile.location);
        if path.is_file() {
            let original_path = path.clone();
            let time = DateTime::<Local>::from(metadata(&path).unwrap().modified().unwrap());
            match time.format("%Y/%m/%d %H:%M:%S").to_string().cmp(&savefile.update_time) {
                Greater => {
                    let file_name 
                        = time.format("%Y%m%d%H%M%S").to_string() + &path.file_name().unwrap().to_string_lossy().to_string();
                    path.pop();
                    path.push("Backups");
                    add_backups(&original_path, &path, file_name);
                },
                _ => continue,
            }
        }
    }
}

fn add_backups(origin_path: &PathBuf, path: &PathBuf, file_name: String) {
    let file_path = &path.join(&file_name);
    if file_path.exists() {
        return;
    }
    let mut backups_file =  Vec::new();
    File::open(origin_path).unwrap().read_to_end(&mut backups_file).unwrap();
    File::create(file_path).unwrap().write_all(&backups_file).unwrap();
}