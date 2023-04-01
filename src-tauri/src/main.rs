// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// use log::LevelFilter;
// use simplelog::{TermLogger, TerminalMode, Config, ColorChoice};

#[tauri::command]
fn greet(){
    println!("testing from cargo");

}

fn main() {
    // let logger = TermLogger::init(
    //     LevelFilter::Trace,
    //     Config::default(),
    //     TerminalMode::Mixed,
    //     ColorChoice::Auto
    // ).unwrap();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
