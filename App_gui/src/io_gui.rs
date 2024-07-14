use noitarchiver_core::utils::io_manager::*;
use tauri::Manager;
use std::fmt::Arguments;
use std::sync::mpsc::{channel, Sender, Receiver};

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

    fn io_comfirm(&self,  args: Arguments) -> bool {
        get_app_handle().emit_all("get_confirm", format!("{}", args)).unwrap();
        
        let (tx, rx): (Sender<bool>, Receiver<bool>) = channel();
        get_app_handle().once_global("confirm", move |event| {
            if let Some(msg) = event.payload() {
                if msg=="true" {
                    tx.send(true).unwrap();
                    return;
                }
            }
            tx.send(false).unwrap();
        });
        rx.recv().unwrap()
    }

    fn io_cls(&self) {
        todo!()
    }
}