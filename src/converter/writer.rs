use std::io::Stdout;
use crate::converter::parser::FormatType;
use crate::errors::ParserError;
use crate::models::camt053::DocumentCamt053;

pub struct OutDataType {
    format_type: FormatType,
    buff_read: Stdout,
}

impl DocumentCamt053 {
    fn to_camt053(&self, buff_write: Stdout) -> Result<(), ParserError> {
        let mut writer = buff_write.lock();
        serde_xml_rs::to_writer(&mut writer, self)?;
        Ok(())
    }
    
    fn to_mt940(&self, buff_write: Stdout) -> Result<(), ParserError> {
        Ok(())
    }
}