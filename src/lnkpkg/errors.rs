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
    // Any additional information passed along, such as the argument name that caused the error
    pub info: Option<Vec<String>>,
}

impl LnkError {
    #[allow(dead_code)]
    pub fn utf16_decode_error(err: String)->Self{
        LnkError {
            message: format!("{}",err),
            kind: ErrorKind::Utf16Error,
            info: Some(vec![]),
        }
    }
}

impl From<FromUtf8Error> for LnkError {
    fn from(err: FromUtf8Error) -> Self {
        LnkError {
            message: format!("{}",err),
            kind: ErrorKind::FromUtf8Error,
            info: None,
        }
    }
}
impl From<ShellItemError> for LnkError {
    fn from(err: ShellItemError) -> Self {
        LnkError {
            message: format!("{}",err.message),
            kind: ErrorKind::ShellItemError,
            info: err.info
        }
    }
}
impl From<io::Error> for LnkError {
    fn from(err: io::Error) -> Self {
        LnkError {
            message: format!("{}",err),
            kind: ErrorKind::IoError,
            info: None,
        }
    }
}

impl Display for LnkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { writeln!(f, "{}", self.message) }
}
