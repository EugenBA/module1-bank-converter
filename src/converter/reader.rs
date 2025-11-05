use std::io::{BufRead, Stdin};
use crate::converter::parser::FormatType;
use crate::errors::ParserError;
use crate::models::camt053::DocumentCamt053;
use crate::models::mt940::DocumentMt940;
use crate::models::csv::{DocumentCsv, RowCsv};
use csv::Reader;



pub struct InputDataType {
    format_type: FormatType,
    buff_read: Stdin,
}

enum Document{
    DocumentCamt053(Result<DocumentCamt053, ParserError>),
    DocumentMt940(Result<DocumentMt940, ParserError>),
    DocumentCsv(Result<DocumentCsv, ParserError>),
    None(ParserError)
}

impl From<InputDataType> for Document {
    fn from(value: InputDataType) -> Self {
        match value.format_type {
            FormatType::Camt053 | FormatType::Xml => {
                Self::DocumentCamt053(DocumentCamt053::from_camt053(value.buff_read))
            },
            FormatType::Mt940 => {
                Self::DocumentMt940(DocumentMt940::from_mt940(value.buff_read))
            }
            FormatType::Csv => {
                Self::DocumentCsv(DocumentCsv::from_csv(value.buff_read))
            },
            _ => { Self::None(ParserError::BadInputFormatFile("Bad input type file".to_string())) }
        }
    }
}

impl DocumentCamt053 {
    pub fn new() -> Self {
        DocumentCamt053::default()
    }
    
    fn from_camt053(buf_read: Stdin) -> Result<Self, ParserError> {
        let mut reader = buf_read.lock();
        let mut xml_str = String::new();
        while let Ok(byte_reader) = reader.read_line(&mut xml_str) {
            if byte_reader == 0 {
                break;
            }
        }
        serde_xml_rs::from_str(&xml_str)?
    }

}

impl DocumentMt940 {
    fn from_mt940(buf_reader: Stdin) -> Result<Self, ParserError> {
        Err(ParserError::BadInputFormatFile("Bad input format file".to_string()))
    }
}

impl DocumentCsv {
    fn from_csv(buf_read: Stdin) -> Result<Self, ParserError> {
        let reader = buf_read.lock();
        let mut csv_document: DocumentCsv = DocumentCsv::new();
        let mut csv_rdr = Reader::from_reader(reader);
        for row in csv_rdr.deserialize() {
            let row_data: RowCsv = row?;
            csv_document.rows.push(row_data);
        }
        Ok(csv_document)
    }
}
