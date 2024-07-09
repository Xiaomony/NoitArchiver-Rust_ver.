// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate noitarchiver_core;
//use noitarchiver_core::utils::io_manager::IOManager;
use noitarchiver_core::Manager;
use noitarchiver_core::utils::com_analyzer;

mod io_gui;
use io_gui::IOGui;


#[tauri::command]
fn get_comlist() -> Vec<com_analyzer::ComMap> {
	com_analyzer::Analyzer::generate_comlist()
}

fn main() {
	
	let logger = IOGui{};
	let mut _manager = Manager::new(&logger);

	tauri::Builder::default()
		.invoke_handler(tauri::generate_handler![get_comlist])
    	.run(tauri::generate_context!())
    	.expect("error while running tauri application");
}
