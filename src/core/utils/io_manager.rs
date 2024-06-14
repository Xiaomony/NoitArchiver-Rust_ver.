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
    ($logger:expr,$format_str:expr $(, $arg:expr)*) => {
        $logger.io_print_suc(format_args!($format_str, $($arg),*))
    };
}
#[macro_export]
macro_rules! outln_suc {
    ($logger:expr,$format_str:expr $(, $arg:expr)*) => {
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
    fn io_getint(&self) -> Option<i32>;
    fn io_comfirm(&self) -> bool;

    fn io_cls(&self);
}

#[derive(Debug)]
pub enum Error {
    GeneralError(String),
    JsonTranslateError(serde_json::Error),
    IoError(std::io::Error),
    ParseIntError(core::num::ParseIntError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::GeneralError(ref msg) => write!(f, "{}", msg),
            Self::IoError(ref err) => write!(f, "[IoError]\t{}", err),
            Self::JsonTranslateError(ref err) => write!(f, "[JsonTranslateError]\t{}", err),
            Self::ParseIntError(ref err) => write!(f, "[ParseIntError]\t{}", err),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::GeneralError(_) => None,
            Self::IoError(ref err) => Some(err),
            Self::JsonTranslateError(ref err) => Some(err),
            Self::ParseIntError(ref err) => Some(err),
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

impl From<core::num::ParseIntError> for Error {
    fn from(value: core::num::ParseIntError) -> Self {
        Error::ParseIntError(value)
    }
}

// 对Result类型加工，方便错误的处理和传播

pub trait ResultExt<T> {
    fn with_msg(self, msg: &str) -> Result<T, Error>;
    fn with_moreinfo(self, msg: &str) -> Result<T, Error>;
}

impl<T, E> ResultExt<T> for Result<T, E>
where
    E: Into<Error>,
{
    fn with_msg(self, msg: &str) -> Result<T, Error> {
        match self {
            Err(_) => Err(Error::GeneralError(format!("{}", msg))),
            Ok(ok) => Ok(ok),
        }
    }
    fn with_moreinfo(self, msg: &str) -> Result<T, Error> {
        self.map_err(|e| Error::GeneralError(format!("{}\n\t{}", msg, e.into())))
    }
}
