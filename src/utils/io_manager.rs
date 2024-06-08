use serde_json;
use std::fmt::Arguments;

// -----------------------------------
#[macro_export]
macro_rules! out {
    ($logger:expr,$format_str:expr $(, $arg:expr)*) => {
        $logger.io_print(format_args!($format_str, $($arg),*))
    };
}
#[macro_export]
macro_rules! outln {
    ($logger:expr,$format_str:expr $(, $arg:expr)*) => {
        $logger.io_print(format_args!(concat!($format_str, "\n"), $($arg),*))
    };
}
// out err
#[macro_export]
macro_rules! out_err {
    ($logger:expr,$format_str:expr $(, $arg:expr)*) => {
        $logger.io_print_err(format_args!($format_str, $($arg),*))
    };
}
#[macro_export]
macro_rules! outln_err {
    ($logger:expr,$format_str:expr $(, $arg:expr)*) => {
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
    ($logger:expr,$format_str:expr $(, $arg:expr)*) => {
        $logger.io_print_log(format_args!($format_str, $($arg),*))
    };
}
#[macro_export]
macro_rules! outln_log {
    ($logger:expr,$format_str:expr $(, $arg:expr)*) => {
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

#[derive(Debug)]
pub enum Error {
    None, // 不便构造Clone方法的变体,在Clone时返回None
    CommandError(String),
    JsonTranslateError(serde_json::Error),
    IoError(std::io::Error),
}

impl Clone for Error {
    fn clone(&self) -> Self {
        match *self {
            Self::CommandError(ref msg) => Self::CommandError(msg.clone()),
            Self::IoError(ref err) => {
                Self::IoError(std::io::Error::new(err.kind(), err.to_string().as_str()))
            }
            Self::JsonTranslateError(_) => Self::None,
            Self::None => Self::None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::None => write!(f, "None"),        // ***********
            Self::CommandError(ref msg) => write!(f, "[Command Error]\t{}", msg),
            Self::IoError(ref err) => write!(f, "[IoError]\t{}", err),
            Self::JsonTranslateError(ref err) => write!(f, "[JsonTranslateError]\t{}", err),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::None => None, // **********
            Self::CommandError(_) => None,
            Self::IoError(ref err) => Some(err),
            Self::JsonTranslateError(ref err) => Some(err),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IoError(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::JsonTranslateError(value)
    }
}
