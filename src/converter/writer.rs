use std::io::Stdout;
use crate::errors::ParserError;
use crate::models::camt053::Document;

impl Document{
    fn to_camt053(&self, buff_write: Stdout) -> Result<(), ParserError> {
        let mut writer = buff_write.lock();
        serde_xml_rs::to_writer(&mut writer, self)?;
        Ok(())
    }
}