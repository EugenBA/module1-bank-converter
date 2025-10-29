use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Error, Debug)]
enum CommandArgsError
{
    BadArgument(String),

}


#[derive(Error, Debug)]
pub enum ParserError
{
    FileReadError(String),
    BadInputFormatFile(String),
    FileWriteError(String),
    BadFormatType(String)
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::FileReadError(s) => write!(f, "File read error: {}", s),
            ParserError::BadInputFormatFile(s) => write!(f, "Bad input format file: {}", s),
            ParserError::FileWriteError(s) => write!(f, "File write error: {}", s),
            ParserError::BadFormatType(s) => write!(f, "Bad format type: {}", s),
        }
    }
}

impl Display for CommandArgsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandArgsError::BadArgument(s) => write!(f, "Bad argument: {}", s),
        }
    }
}


