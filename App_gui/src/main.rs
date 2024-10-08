// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use once_cell::sync::OnceCell;
use tauri::Manager;
use std::sync::Mutex;
use std::{env, thread};

extern crate noitarchiver_core;
use noitarchiver_core::Manager as nManager;
use noitarchiver_core::utils::com_analyzer;
use noitarchiver_core::utils::file_manager::ArchiveInfo;

mod io_gui;
use io_gui::IOGui;

static LOGGER:IOGui = IOGui{};
static MANAGER: OnceCell<Mutex<nManager<'static, IOGui>>> = OnceCell::new();
static APP_HANDLE: OnceCell<tauri::AppHandle> = OnceCell::new();
fn get_app_handle() -> &'static tauri::AppHandle {
    APP_HANDLE.get().expect("AppHandle not set")
}

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

//#[tauri::command]
fn run_command(command: &str) {
	let mut manager = MANAGER.get().unwrap().lock().unwrap();
	let cuted = &(&command[1..=command.len()-2]).replace("\\\"", "\"");
	println!("run command: {}",cuted);
	manager.run_command(cuted).unwrap();
}

#[tauri::command]
fn get_help_str() -> String {
	nManager::<IOGui>::HELP_MSG.to_string()
}

#[tauri::command]
fn get_app_version() -> String {
	env!("CARGO_PKG_VERSION").to_string()
}

fn main() {
	let _ = MANAGER.set(Mutex::new(nManager::new(&LOGGER).unwrap()));

	let app = tauri::Builder::default()
		.setup(|app| {
			APP_HANDLE.set(app.handle().clone()).unwrap();
			app.listen_global("run_command", |event| {
				thread::spawn(move || {
					run_command(event.payload().unwrap());
					get_app_handle().emit_all("fresh_arch", {}).unwrap();
				});
			});
			Ok(())
		})
		.invoke_handler(tauri::generate_handler![get_comlist, get_archinfos, get_help_str, get_app_version])
		.build(tauri::generate_context!())
		.expect("error while build tauri application");

	app.run(|_app_handle, event| match event {
		tauri::RunEvent::ExitRequested { .. } => {
			//api.prevent_exit();
		}
		_ => {}
	});
}
