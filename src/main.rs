use chrono::{DateTime, Local};
use clap::Parser;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long, value_parser)]
    directory_name: String,
}

fn get_permission_txt(mode: &u32) -> String {
    let mut permission_txt = String::new();

    let mode_oct = format!("{:o}", mode);
    let mode_int = mode_oct.parse::<i32>().unwrap() % 1000;
    let mod_file = mode_oct.parse::<i32>().unwrap() / 1000;

    let file_type = match mod_file {
        100 => "-",
        40 => "d",
        other => "",
    };
    permission_txt.push_str(file_type);

    let file_code = mode_int.to_string();
    for x in file_code.chars() {
        let code = match x {
            '0' => "-",
            '1' => "-x",
            '2' => "-w-",
            '3' => "-wx",
            '4' => "r-",
            '5' => "r-x",
            '6' => "rw-",
            '7' => "rwx",
            other => " ",
        };
        permission_txt.push_str(code);
    }

    permission_txt
}

fn get_name(file: PathBuf) -> String {
    let path_name = file
        .into_os_string()
        .into_string()
        .unwrap_or("".to_string());
    let path_last = path_name.trim().split('/').last().unwrap_or("").to_string();
    path_last
}

fn main() {
    let matches = Args::parse();
    let search = matches.directory_name;
    let entries = fs::read_dir(search).unwrap();
    for entry in entries {
        let entry = entry.unwrap();
        let metadata = entry.metadata().unwrap();
        let mode = metadata.permissions().mode();
        let permission_txt = get_permission_txt(&mode);
        let modified = metadata.modified().unwrap();
        let modified_time: DateTime<Local> = DateTime::from(modified);
        let name = get_name(entry.path());
        println!(
            "{}, {}, {}",
            permission_txt,
            modified_time.format("%_d %b %H:%M").to_string(),
            name,
        );
    }
}
