use std::io::{BufRead, Stdin};
use crate::converter::parser::FormatType;
use crate::errors::ParserError;
use crate::models::camt053::{BkToCstmAttribute, DocumentCamt053};
use crate::models::mt940::{DocumentMt940};
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

    fn find_record(document: &str) -> Option<Vec<(usize, usize)>> {
        let mut vec_start_pattern: Vec<usize> = Vec::new();
        let mut vec_end_pattern: Vec<usize> = Vec::new();
        let mut records: Vec<(usize, usize)> = Vec::new();
        for (index, found_pattern) in document.match_indices("{1"){
            vec_start_pattern.push(index);
        }
        for (index, found_pattern) in document.match_indices("{5:"){
            vec_end_pattern.push(index);
        }
        for i in vec_start_pattern.iter().enumerate(){
            records.push((vec_start_pattern[i.0], vec_end_pattern[i.0]));
        }
        Some(records)
    }

    fn parse_one_record(document: &str) -> Option<BkToCstmAttribute> {
        let mut record: BkToCstmAttribute = BkToCstmAttribute::default();
        None
    }

    fn from_mt940(buf_read: Stdin) -> Result<Self, ParserError> {
        let mut regex_pattern = String::new();
        match DocumentMt940::find_record(&regex_pattern) {
            Some(records) => {
                let mut document = DocumentMt940::default();
                for record in records {

                }
            }
            _ => {
               return Err(ParserError::BadInputFormatFile("No \
               find start or end patter in file".to_string()));
            }
        }
        Err(ParserError::BadInputFormatFile("No implement parse document".to_string()))
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
