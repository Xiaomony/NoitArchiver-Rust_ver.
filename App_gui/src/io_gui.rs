use noitarchiver_core::utils::io_manager::*;
use std::fmt::Arguments;

pub struct IOGui {}

impl IOManager for IOGui {
    fn io_print(&self, args: Arguments) {
        todo!()
    }

    fn io_print_err(&self, args: Arguments) {
        todo!()
    }

    fn io_print_warn(&self, args: Arguments) {
        todo!()
    }

    fn io_print_log(&self, args: Arguments) {
        todo!()
    }

    fn io_print_suc(&self, args: Arguments) {
        todo!()
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