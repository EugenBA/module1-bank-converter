
use std::io::{BufRead, BufReader, Stdin};
use crate::models::camt053::*;
use crate::errors::{ParserError};

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

    fn from_csv(buf_reader: Stdin) -> Result<Self>{

    }
    fn from_camt053(buff_read: Stdin) -> Result<Self> {


    }

    fn from_mt940(buf_reader: Stdin) -> Result<Self> {}


}



impl From<InputDataType> for Document {
    fn from(value: InputDataType) -> Result<Self> {
        match value.format_type {
            FormatType::Camt053 | FormatType::Xml =>{ Document::from_camt053(value.buff_read)},
            FormatType::Mt940 => { Document::from_mt940(value.buff_read)},
            FormatType::Csv => { Document::from_csv(value.buff_read)
            },
            _ => { Err(ParserError::BadInputFormatFile(String::from("Unsupported format")))}
        }
    }
}