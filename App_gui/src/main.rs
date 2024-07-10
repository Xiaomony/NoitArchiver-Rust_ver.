// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use once_cell::sync::OnceCell;

extern crate noitarchiver_core;
use noitarchiver_core::Manager as nManager;
use noitarchiver_core::utils::com_analyzer;
use noitarchiver_core::utils::file_manager::ArchiveInfo;

mod io_gui;
use io_gui::IOGui;

static LOGGER:IOGui = IOGui{};
static MANAGER: OnceCell<nManager<'static, IOGui>> = OnceCell::new();

#[tauri::command]
fn get_comlist() -> Vec<com_analyzer::ComMap> {
	com_analyzer::Analyzer::generate_comlist()
}
#[tauri::command]
fn get_archinfos() -> Vec<ArchiveInfo> {
	let manager = MANAGER.get().unwrap();
	let infos = manager.file_manager.get_archive_infos().clone();
	infos
}

fn main() {
	MANAGER.set(nManager::new(&LOGGER).unwrap());
	let logger = IOGui {};
	let m = nManager::new(&logger).unwrap();
	println!("{:?}", m.file_manager.get_archive_infos());

	tauri::Builder::default()
		.invoke_handler(tauri::generate_handler![get_comlist, get_archinfos])
		//.invoke_handler(tauri::generate_handler![])
    	.run(tauri::generate_context!())
    	.expect("error while running tauri application");
}