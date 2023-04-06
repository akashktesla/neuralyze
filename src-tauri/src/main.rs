#![allow(warnings)]
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use neuralyze::rustpp::write_file;
use std::fs::{write,File};
use std::io::Read;
use std::env;


#[tauri::command]
fn save(path:String,data:String){
    println!("To save: {} on {}",data,path);
    match write(path,data){
        Ok(_) =>(),
        Err(_) => println!("error writing file") ,
    }
}
#[tauri::command]
fn load(path:String)->String{
    let mut _file = File::open(path).expect("can't open file");
    let mut contents = String::new();
    _file.read_to_string(&mut contents).expect("can't read the file");
    contents = format!(r#"{}"#,contents); 
    return contents
}

#[tauri::command]
fn dp()->String{
    return env!("NZ_DP").to_string()
}


#[tauri::command]
fn print(data:String){
    println!("{}",data);
}


fn main() {
    // let logger = TermLogger::init(
    //     LevelFilter::Trace,
    //     Config::default(),
    //     TerminalMode::Mixed,
    //     ColorChoice::Auto
    // ).unwrap();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![save,load,print,dp])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}







