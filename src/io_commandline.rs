use noita_archive_manager::utils::io_manager::*;
use std::fmt::Arguments;
use std::io::{self, Write};
use std::process::Command;

use colored::*;

pub struct IOCommandLine {}

impl IOCommandLine {
    pub fn new() -> Self {
        Self {}
    }
}

impl IOCommandLine {
    pub fn flush_buffer() {
        io::stdout().flush().unwrap();
    }
}

impl IOManager for IOCommandLine {
    fn io_print(&self, args: Arguments) {
        print!("{}", args);
        Self::flush_buffer();
    }
    fn io_print_err(&self, args: Arguments) {
        print!("{}", format!("{}", args).red());
        Self::flush_buffer();
    }
    fn io_print_warn(&self, args: Arguments) {
        print!("{}", format!("{}", args).yellow());
        Self::flush_buffer();
    }
    fn io_print_log(&self, args: Arguments) {
        print!("{}", format!("{}", args).blue());
        Self::flush_buffer();
    }
    fn io_print_suc(&self, args: Arguments) {
        print!("{}", format!("{}", args).green());
        Self::flush_buffer();
    }

    fn io_getline(&self) -> String {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("fail to read a line");
        input
    }

    fn io_cls(&self) {
        Command::new("clear")
            .status()
            .expect("Failed to clear screen");
    }
}
