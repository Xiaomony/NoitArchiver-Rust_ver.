use noita_archive_manager::utils::io_manager::*;
use std::fmt::Arguments;
use std::process::Command;

use colored::*;

pub struct IOCommandLine {}

impl IOCommandLine {
    pub fn new() -> Self {
        Self {}
    }
}

impl IOManager for IOCommandLine {
    fn io_print(&self, args: Arguments) {
        print!("{}", args);
    }
    fn io_print_err(&self, args: Arguments) {
        print!("{}", format!("{}", args).red());
    }
    fn io_print_warn(&self, args: Arguments) {
        print!("{}", format!("{}", args).yellow());
    }
    fn io_print_log(&self, args: Arguments) {
        print!("{}", format!("{}", args).blue());
    }
    fn io_print_suc(&self, args: Arguments) {
        print!("{}", format!("{}", args).green());
    }

    fn io_getline(&self) -> String {
        "   ".to_string()
    }

    fn io_cls(&self) {
        Command::new("clear")
            .status()
            .expect("Failed to clear screen");
    }
}
