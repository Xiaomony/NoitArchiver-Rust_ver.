// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use once_cell::sync::OnceCell;
use std::sync::Mutex;

extern crate noitarchiver_core;
use noitarchiver_core::Manager as nManager;
use noitarchiver_core::utils::com_analyzer;
use noitarchiver_core::utils::file_manager::ArchiveInfo;

mod io_gui;
use io_gui::IOGui;

static LOGGER:IOGui = IOGui{};
static MANAGER: OnceCell<Mutex<nManager<'static, IOGui>>> = OnceCell::new();

#[tauri::command]
fn get_comlist() -> Vec<com_analyzer::ComMap> {
	com_analyzer::Analyzer::generate_comlist()
}
#[tauri::command]
fn get_archinfos() -> Vec<ArchiveInfo> {
	let manager = MANAGER.get().unwrap().lock().unwrap();
	let infos = manager.file_manager.get_archive_infos().clone();
	infos
}

#[tauri::command]
fn run_command(command: &str) {
	let mut manager = MANAGER.get().unwrap().lock().unwrap();
	manager.run_command(command).unwrap();
}

#[tauri::command]
fn get_help_str() -> String {
	nManager::<IOGui>::HELP_MSG.to_string()
}

fn main() {
	let _ = MANAGER.set(Mutex::new(nManager::new(&LOGGER).unwrap()));

	tauri::Builder::default()
		.invoke_handler(tauri::generate_handler![get_comlist, get_archinfos, run_command, get_help_str])
    	.run(tauri::generate_context!())
    	.expect("error while running tauri application");
}
