#![allow(warnings)]
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use neuralyze::rustpp::write_file;
use std::fs::write;

#[tauri::command]
fn save(path:String,data:String){
    println!("To save: {} on {}",data,path);
    match write(path,data){
        Ok(_) =>(),
        Err(_) => println!("error writing file") ,
    }
}

fn main() {
    // let logger = TermLogger::init(
    //     LevelFilter::Trace,
    //     Config::default(),
    //     TerminalMode::Mixed,
    //     ColorChoice::Auto
    // ).unwrap();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![save])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
