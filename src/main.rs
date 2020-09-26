mod model_version_1;
mod model_version_2;

pub use crate::model_version_1::SaveVersion1;
pub use crate::model_version_2::downgrade_to_v1;
pub use crate::model_version_2::upgrade_to_v2;
pub use crate::model_version_2::SaveVersion2;
use platform_dirs::{AppDirs, AppUI};
use serde_json::Value;
use std::env;
use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use std::io::{self, BufReader, Read};
use std::time::SystemTime;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        let verb = &args[1];

        match verb.as_ref() {
            "update" => update_cmd(),
            "help" => help_cmd(false, ""),
            "downgrade" => {
                if args.len() >= 3 {
                    let allow_downgrade = confirm(
                        "Do you really want to downgrade your save? \
                        You may experience data loses",
                    )
                    .unwrap();
                    if allow_downgrade {
                        let version: u64 = args[2]
                            .parse::<u64>()
                            .expect("ERROR: Cannot get version number");
                        downgrade_cmd(version);
                    } else {
                        println!("Operation aborted");
                    }
                } else {
                    help_cmd(true, "ERROR: The version number is missing")
                }
            }
            _ => help_cmd(true, "ERROR: This verb is not allowed"),
        }
    } else {
        help_cmd(true, "ERROR: The verb arg is missing");
    }
}

fn help_cmd(has_error: bool, err: &str) {
    if has_error {
        eprintln!("{}", err);
        println!("");
    }
    println!("Usage: save-updater.exe <verb> [args]\n");
    println!("List of verbs:");
    println!("   update                Update the Chronos file save");
    println!("   downgrade <version>   Downgrade to a version");
    println!("   help                  Show this screen");
}

fn downgrade_cmd(version: u64) {
    let json_file = open_save_file().expect("ERROR: Cannot open save file");
    let v: Value = serde_json::from_str(&json_file).expect("ERROR: Cannot parse save file");
    let current_version: u64 = v["version"]
        .as_u64()
        .expect("ERROR: Error while parsing save version");
    if version == (current_version - 1) && version > 0 {
        add_to_backup(&json_file).expect("ERROR: Failed to create backup");
        match version {
            1 => {
                let s: SaveVersion2 =
                    serde_json::from_str(&json_file).expect("ERROR: Cannot parse save file");
                let new_save = downgrade_to_v1(s);
                let json =
                    serde_json::to_string(&new_save).expect("ERROR: Fail to build new save file");
                save_to_file(&json, "data.json").expect("ERROR: Fail to save updated file");
                println!("INFO: Save file has been downgraded");
            }
            _ => eprintln!("ERROR: Version not supported"),
        }
    } else {
        eprintln!("ERROR: Only downgrading to the previous version is allowed");
    }
}

fn update_cmd() {
    let json_file = open_save_file().expect("ERROR: Cannot open save file");
    let v: Value = serde_json::from_str(&json_file).expect("ERROR: Cannot parse save file");
    if v["version"] == 1 {
        add_to_backup(&json_file).expect("ERROR: Failed to create backup");
        let s: SaveVersion1 =
            serde_json::from_str(&json_file).expect("ERROR: Cannot parse save file");
        let new_save = start_upgrade_from_v1(s);
        let json = serde_json::to_string(&new_save).expect("ERROR: Fail to build new save file");
        save_to_file(&json, "data.json").expect("ERROR: Fail to save updated file");
        println!("INFO: Save file is up-to-date");
    } else {
        println!("Save already up-to-date");
    }
}

fn open_save_file() -> std::result::Result<std::string::String, &'static str> {
    let app_dirs = AppDirs::new(Some("Chronos"), AppUI::CommandLine)
        .expect("ERROR: Cannot get application data folder");
    let path = app_dirs.config_dir.join("data").join("data.json");
    if path.exists() {
        let file = File::open(path).expect("ERROR: cannot open file");
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader
            .read_to_string(&mut contents)
            .expect("cannot read file");
        Ok(contents)
    } else {
        Err("Save file not found")
    }
}

fn save_to_file(content: &String, filename: &str) -> std::io::Result<()> {
    let app_dirs = AppDirs::new(Some("Chronos"), AppUI::CommandLine)
        .expect("ERROR: Cannot get application data folder");
    let path = app_dirs.config_dir.join("data").join(filename);
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn confirm(msg: &str) -> std::io::Result<bool> {
    println!("{}\nType 'yes', other input will be ignore", msg);
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim() == "yes")
}

fn add_to_backup(content: &String) -> std::io::Result<()> {
    let app_dirs = AppDirs::new(Some("Chronos"), AppUI::CommandLine)
        .expect("ERROR: Cannot get application data folder");
    let path = app_dirs.config_dir.join("data").join("backup");
    create_dir_all(&path)?;
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("ERROR: Failed to get timestamp")
        .as_secs();
    let path = path.join(format!("{}.data.json", timestamp));
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn start_upgrade_from_v1(save: SaveVersion1) -> SaveVersion2 {
    println!("INFO: Starting upgrading from version 1 to version 2");
    let save = upgrade_to_v2(save);
    save
}
