use rshellitems::errors::{ShellItemError};
use std::string::FromUtf8Error;
use std::fmt;
use std::fmt::Display;
use std::io;

#[derive(Debug)]
pub enum ErrorKind {
    IoError,
    ShellItemError,
    FromUtf8Error,
    Utf16Error
}

// Lnk Parsing Error
#[derive(Debug)]
pub struct LnkError {
    // Formated error message
    pub message: String,
    // The type of error
    pub kind: ErrorKind,
    pub trace: String
}

impl LnkError {
    #[allow(dead_code)]
    pub fn utf16_decode_error(err: String)->Self{
        LnkError {
            message: format!("{}",err),
            kind: ErrorKind::Utf16Error,
            trace: backtrace!()
        }
    }
}

impl From<FromUtf8Error> for LnkError {
    fn from(err: FromUtf8Error) -> Self {
        LnkError {
            message: format!("{}",err),
            kind: ErrorKind::FromUtf8Error,
            trace: backtrace!()
        }
    }
}
impl From<ShellItemError> for LnkError {
    fn from(err: ShellItemError) -> Self {
        LnkError {
            message: format!("{}",err.message),
            kind: ErrorKind::ShellItemError,
            trace: format!("{}",err.trace)
        }
    }
}
impl From<io::Error> for LnkError {
    fn from(err: io::Error) -> Self {
        LnkError {
            message: format!("{}",err),
            kind: ErrorKind::IoError,
            trace: backtrace!()
        }
    }
}

impl Display for LnkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "message: {}\nkind: {:?}\n{}",
            self.message, self.kind, self.trace
        )
    }
}
