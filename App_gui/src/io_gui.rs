use noitarchiver_core::utils::io_manager::*;
use tauri::Manager;
use std::fmt::Arguments;

use crate::get_app_handle;

pub struct IOGui {}

impl IOManager for IOGui {
    fn io_print(&self, args: Arguments) {
        get_app_handle().emit_all("out_common", format!("{}", args)).unwrap();
    }

    fn io_print_err(&self, args: Arguments) {
        get_app_handle().emit_all("out_err", format!("{}", args)).unwrap();
    }

    fn io_print_warn(&self, args: Arguments) {
        get_app_handle().emit_all("out_warn", format!("{}", args)).unwrap();
    }

    fn io_print_log(&self, args: Arguments) {
        get_app_handle().emit_all("out_log", format!("{}", args)).unwrap();
    }

    fn io_print_suc(&self, args: Arguments) {
        get_app_handle().emit_all("out_suc", format!("{}", args)).unwrap();
    }

    fn io_getline(&self) -> String {
        todo!()
    }

    fn io_getint(&self) -> Option<i32> {
        todo!()
    }

    fn io_comfirm(&self) -> bool {
        todo!()
    }

    fn io_cls(&self) {
        todo!()
    }
}