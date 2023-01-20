use std::{error::Error, fmt, path::Path};

#[derive(Debug)]
pub struct FeoError {
    id: ErrorId,
    info: ErrorInfo,
}

impl fmt::Display for FeoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {:?}", self.id, self.source().unwrap())
    }
}

impl Error for FeoError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.info)
    }
}

struct ErrorInfo {
    char: Option<char>,
    file: &'static Path,
    line: usize,
    col: usize,
    message: &'static str,
}

impl fmt::Debug for ErrorInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ErrorInfo")
            .field("char", &self.char)
            .field("file", &self.file)
            .field("line", &self.line)
            .field("col", &self.col)
            .field("message", &self.message)
            .finish()
    }
}

impl fmt::Display for ErrorInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let position = format!("{}, {}:{}", self.file.display(), self.line, self.col);

        write!(
            f,
            "`{}`. {} ({}).",
            self.char.unwrap(),
            self.message,
            position
        )
    }
}

impl Error for ErrorInfo {}

#[derive(Debug)]
pub enum ErrorId {
    InvalidChar,
    InvalidData,
    Unknown,
}

impl fmt::Display for ErrorId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ErrorId::InvalidChar => write!(f, "invalid character"),
            ErrorId::InvalidData => write!(f, "invalid data"),
            ErrorId::Unknown => write!(f, "unknown error"),
        }
    }
}

impl Error for ErrorId {}

impl FeoError {
    pub fn new(
        id: ErrorId,
        char: Option<char>,
        file: &'static Path,
        line: usize,
        col: usize,
        message: &'static str,
    ) -> FeoError {
        let info = ErrorInfo {
            char,
            file,
            line,
            col,
            message,
        };

        FeoError { id, info }
    }
}
