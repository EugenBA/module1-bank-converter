use std::io::{Write};
use csv::{Writer};
use crate::errors::{ConvertError};
use crate::models::camt053::{DocumentCamt053};
use crate ::models::mt940::{DocumentMt940};
use crate::models::csv::{DocumentCsv};


impl DocumentCamt053 {
    pub fn write_to<W: Write>(&mut self, writer: &mut W) -> Result<(), ConvertError> {
        serde_xml_rs::to_writer(writer, &self)?;
        Ok(())
    }
}

impl DocumentMt940 {
    pub fn write_to<W: Write>(&mut self, writer: &mut W) -> Result<(), ConvertError> {
        let mut record_write = String::new();
        for record in &self.document {
            record_write.push_str("{1:F01");
            record_write.push_str(&record.stmt.acct.svcr.fin_inst_id.bic);
            record_write.push_str("}\n");
            record_write.push_str("{2:");
            record_write.push_str(&record.grp_hdr.msg_id);
            record_write.push_str("}\n{3:}\n{4:\n");
            record_write.push_str(":20:");
            record_write.push_str(&record.grp_hdr.msg_id);
            record_write.push_str("\n");
            record_write.push_str(":25:");
            record_write.push_str(&record.stmt.acct.ownr.id.org_id.othr.id);
            record_write.push_str("\n");
            record_write.push_str(":28C:");
            record_write.push_str(&record.stmt.electr_seq_nb);
            record_write.push_str("/");
            record_write.push_str(&record.stmt.lgl_seq_nv);
            record_write.push_str("\n");
            DocumentMt940::extract_field_6x_mt940(&record, &mut record_write);
            DocumentMt940::extract_field_61_86_mt940(&record.stmt.ntry, &mut record_write);
            record_write.push_str("}\n{5:-}\n");
            writer.write_all(record_write.as_bytes())?;
            writer.flush()?;
            record_write.clear();
        }
        Ok(())
    }
}

impl DocumentCsv {
    pub fn write_to<W: std::io::Write>(&mut self, writer: &mut W) -> Result<(), ConvertError> {
        let mut csv_wrt = Writer::from_writer(writer);
        for row in &self.rows {
            csv_wrt.serialize(row)?;
            csv_wrt.flush()?;
        }
        Ok(())
    }
}

/*
impl Document {
    pub(crate) fn to(document: DocumentType, out: OutDataType) -> Result<(), ConvertError> {
        let mut wr_data = WriteData{ camt_mt: DocumentCamt053::default() };

        match document {
            DocumentType::DocumentCamt053(camt054) => {
                wr_data.camt_mt = camt054;
            },
            DocumentType::DocumentMt940(mt940) => {
                wr_data.camt_mt.bk_to_cstm = mt940.document;
            },
            DocumentType::DocumentCsv(csv) => {},
        }
        if let Some(buff_write) = out.buff_write {
            match out.format_type {
                FormatType::None => {}
                FormatType::Csv => {}
                FormatType::Mt940 => {
                    wr_data.to_mt940(buff_write)?;
                }
                FormatType::Camt053 | FormatType::Xml => {
                    wr_data.to_camt053(buff_write)?;
                }
            }
        }
        Ok(())
    }
}

*/