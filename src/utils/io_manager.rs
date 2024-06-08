use std::fmt::Arguments;
// -----------------------------------
#[macro_export]
macro_rules! out {
    ($logger:expr,$format_str:expr $(, $arg:expr),*) => {
        $logger.io_print(format_args!($format_str, $($arg),*))
    };
}
#[macro_export]
macro_rules! outln {
    ($logger:expr,$format_str:expr $(, $arg:expr),*) => {
        $logger.io_print(format_args!(concat!($format_str, "\n"), $($arg),*))
    };
}
// out err
#[macro_export]
macro_rules! out_err {
    ($logger:expr,$format_str:expr $(, $arg:expr),*) => {
        $logger.io_print_err(format_args!($format_str, $($arg),*))
    };
}
#[macro_export]
macro_rules! outln_err {
    ($logger:expr,$format_str:expr $(, $arg:expr),*) => {
        $logger.io_print_err(format_args!(concat!($format_str, "\n"), $($arg),*))
    };
}
// out warn
#[macro_export]
macro_rules! out_warn {
    ($logger:expr,$format_str:expr $(, $arg:expr)*) => {
        $logger.io_print_warn(format_args!($format_str, $($arg),*))
    };
}
#[macro_export]
macro_rules! outln_warn {
    ($logger:expr,$format_str:expr $(, $arg:expr)*) => {
        $logger.io_print_warn(format_args!(concat!($format_str, "\n"), $($arg),*))
    };
}
// out log
#[macro_export]
macro_rules! out_log {
    ($logger:expr,$format_str:expr $(, $arg:expr),*) => {
        $logger.io_print_log(format_args!($format_str, $($arg),*))
    };
}
#[macro_export]
macro_rules! outln_log {
    ($logger:expr,$format_str:expr $(, $arg:expr),*) => {
        $logger.io_print_log(format_args!(concat!($format_str, "\n"), $($arg),*))
    };
}
// out suc
#[macro_export]
macro_rules! out_suc {
    ($logger:expr,$format_str:expr $(, $arg:expr),*) => {
        $logger.io_print_suc(format_args!($format_str, $($arg),*))
    };
}
#[macro_export]
macro_rules! outln_suc {
    ($logger:expr,$format_str:expr $(, $arg:expr),*) => {
        $logger.io_print_suc(format_args!(concat!($format_str, "\n"), $($arg),*))
    };
}
// -----------------------------------
pub trait IOManager {
    fn io_print(&self, args: Arguments);
    fn io_print_err(&self, args: Arguments);
    fn io_print_warn(&self, args: Arguments);
    fn io_print_log(&self, args: Arguments);
    fn io_print_suc(&self, args: Arguments);

    fn io_getline(&self) -> String;

    fn io_cls(&self);
}

pub struct Error<'a, T: IOManager> {
    msg: String,
    logger: &'a T,
}

impl<'a, T: IOManager> Error<'a, T> {
    fn new(msg: &str, logger: &'a T) -> Self {
        Self {
            msg: msg.to_string(),
            logger,
        }
    }
}

impl<'a, T: IOManager> std::fmt::Display for Error<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}
