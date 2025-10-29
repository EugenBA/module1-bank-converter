use std::io::{BufRead, BufReader, Stdin};
use crate::errors::{ParserError};
use crate::models::camt053::*;

pub enum FormatType {
    None,
    Csv,
    Xml,
    Mt940,
    Camt053,
}

pub struct InputDataType {
    format_type: FormatType,
    buff_read: Stdin,
}

impl Document {
    pub fn new() -> Self {
        Document::default()
    }
    pub(crate) fn default() -> Self {
        todo!()
    }
    
    fn from_csv(buf_reader: Stdin) -> Result<Self, ParserError>{
        Err(ParserError::BadInputFormatFile("Bad input format file".to_string()))
    }
    fn from_camt053(buff_read: Stdin) -> Result<Self, ParserError> {
        let mut reader = buff_read.lock();
        let mut xml_str = String::new();
        while let Ok(byte_reader) = reader.read_line(&mut xml_str) {
            if byte_reader == 0 {
                break;
            }
        }
        match serde_xml_rs::from_str(&xml_str) {
            Ok(doc) => {
                Ok(doc)
            },
            Err(e) => {
                Err(ParserError::BadInputFormatFile(e.to_string()))
            }
        }
    }
    
    fn from_mt940(buf_reader: Stdin) -> Result<Self, ParserError> {
        Err(ParserError::BadInputFormatFile("Bad input format file".to_string()))
    }
    
    
}


impl TryFrom<InputDataType> for Document {
    type Error = ParserError;
    fn try_from(value: InputDataType) -> Result<Self, ParserError> {
        match value.format_type {
            FormatType::Camt053 | FormatType::Xml =>{ Document::from_camt053(value.buff_read)},
            FormatType::Mt940 => { Document::from_mt940(value.buff_read)},
            FormatType::Csv => { Document::from_csv(value.buff_read)
            },
            _ => { Err(ParserError::BadInputFormatFile("Bad input format file".to_string()))}
        }
    }
}