use std::fmt::Arguments;

macro_rules! output {
    ($($arg:expr),*) => {
        otl_print(format_args!($($arg),*))
    };
}

macro_rules! output_err {
    ($($arg:expr),*) => {
        otl_print_err(format_args!($($arg),*))
    };
}

macro_rules! output_warn {
    ($($arg:expr),*) => {
        otl_print_warn(format_args!($($arg),*))
    };
}

macro_rules! output_log {
    ($($arg:expr),*) => {
        otl_print_log(format_args!($($arg),*))
    };
}

macro_rules! output_suc {
    ($($arg:expr),*) => {
        otl_print_suc(format_args!($($arg),*))
    };
}

pub trait OutLogger {
    fn otl_print(&self, args: Arguments);
    fn otl_print_err(&self, args: Arguments);
    fn otl_print_warn(&self, args: Arguments);
    fn otl_print_log(&self, args: Arguments);
    fn otl_print_suc(&self, args: Arguments);
}

struct Error {
    msg: String,
}

impl Error {
    fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_string(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}
