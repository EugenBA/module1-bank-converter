use std::fmt::{Display, Formatter};
use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
enum CommandArgsError
{
    BadArgument(String),
}


#[derive(Error, Debug, Deserialize)]
pub enum ParserError
{
    FileReadError(String),
    BadInputFormatFile(String),
    FileWriteError(String),
    BadFormatType(String),
    BadCsvDeserializeError(String),
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::FileReadError(s) => write!(f, "File read error: {}", s),
            ParserError::BadInputFormatFile(s) => write!(f, "Bad input format file: {}", s),
            ParserError::FileWriteError(s) => write!(f, "File write error: {}", s),
            ParserError::BadFormatType(s) => write!(f, "Bad format type: {}", s),
            ParserError::BadCsvDeserializeError(s) => write!(f, "Csv format deserialize error: {}", s),
        }
    }
}

impl  From<serde_xml_rs::Error> for ParserError{
    fn from(err: serde_xml_rs::Error) -> Self{
        ParserError::BadFormatType(err.to_string())
    }
}

impl  From<csv::Error> for ParserError {
    fn from(err: csv::Error) -> Self {
        ParserError::BadCsvDeserializeError(err.to_string())
    }
}


impl Display for CommandArgsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandArgsError::BadArgument(s) => write!(f, "Bad argument: {}", s),
        }
    }
}


