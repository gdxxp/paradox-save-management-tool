use std::{path::PathBuf, fs::{File, remove_file}, io::{Read, Write}};

use tauri::command;

#[command(rename_all = "snake_case")]
pub fn replace_savefile(replaced_item: String, replace_item: String) -> Result<String, String> {
    let replaced_path = PathBuf::from(&replaced_item);
    let replace_path = PathBuf::from(&replace_item);
    let mut replace_file =  Vec::new();
    File::open(replace_path).map_err(|err| err.to_string())?
        .read_to_end(&mut replace_file)
        .map_err(|err| err.to_string())?;
    File::create(replaced_path).map_err(|err| err.to_string())?
        .write_all(&replace_file)
        .map_err(|err| err.to_string())?;
    Ok("success".to_string())
}

#[command(rename_all = "snake_case")]
pub fn delete_savefile(delete_item: String) -> Result<String, String> {
    let delete_path = PathBuf::from(&delete_item);
    match remove_file(delete_path) {
        Ok(_) => Ok("success".to_string()),
        Err(err) => Err(err.to_string()),
    }
}

#[command(rename_all = "snake_case")]
pub fn back_up_savefile(back_up_item: String) -> Result<String, String> {
    let original_path = PathBuf::from(&back_up_item);
    let mut backup_file = Vec::new();
    let mut backup_path = PathBuf::from(&back_up_item);
    backup_path.pop();
    if back_up_item.contains("Stellaris") {
        backup_path.pop();
        print!("yes");
    }
    backup_path.push("Backups");
    create_backup_filename(&mut backup_path, &original_path.file_name().unwrap().to_string_lossy().to_string(), 1);
    File::open(original_path).map_err(|err| err.to_string())?
        .read_to_end(&mut backup_file)
        .map_err(|err| err.to_string())?;
    File::create(backup_path).map_err(|err| err.to_string())?
        .write_all(&backup_file)
        .map_err(|err| err.to_string())?;
    Ok("success".to_string())
}

fn create_backup_filename(backup_path: &mut PathBuf, filename: &String, mut number: i32) {
    let prefix = format!("Backup{} ", number);
    let path = backup_path.join(format!("{}{}", prefix, filename));

    if path.exists() {
        number += 1;
        create_backup_filename(backup_path, filename, number);
    } else {
        backup_path.push(format!("{}{}", prefix, filename));
    }
}