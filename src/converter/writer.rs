use std::io::Stdout;
use crate::converter::parser::FormatType;
use crate::converter::reader::Document;
use crate::errors::ParserError;
use crate::models::camt053::DocumentCamt053;
use crate::models::csv::DocumentCsv;
use crate::models::mt940::DocumentMt940;
use crate::converter::reader::DocumentType;

pub struct OutDataType {
    pub format_type: FormatType,
    pub(crate) buff_write: Option<Stdout>,
}

impl DocumentCamt053 {
    fn to_camt053(&self, buff_write: Stdout) -> Result<(), ParserError> {
        let mut writer = buff_write.lock();
        serde_xml_rs::to_writer(&mut writer, self)?;
        Ok(())
    }
}

impl  DocumentMt940 {
    fn to_mt940(&self, buff_write: Stdout) -> Result<(), ParserError> {
        Ok(())
    }
}


impl DocumentCsv {
    fn to_csv(&self, buff_write: Stdout) -> Result<(), ParserError> {
        Ok(())
    }

}

impl Document {
    pub(crate) fn to(document: DocumentType, out: OutDataType) -> Result<(), ParserError> {
        if let Some(buff_write) = out.buff_write {
            match document {
                DocumentType::DocumentCamt053(camt054) => {
                    camt054.to_camt053(buff_write)?
                },
                DocumentType::DocumentMt940(mt940) => {
                    mt940.to_mt940(buff_write)?
                },
                DocumentType::DocumentCsv(csv) => {
                    csv.to_csv(buff_write)?
                },
            }
        }
        Ok(())
    }
}

