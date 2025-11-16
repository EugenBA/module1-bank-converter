use std::io::{Stdout, Write};
use crate::converter::parser::FormatType;
use crate::converter::reader::Document;
use crate::errors::{ConvertError};
use crate::models::camt053::{BkToCstmAttribute, DocumentCamt053, NtryAttribute};
use crate::models::writabledata::WritableData;
use crate::converter::reader::DocumentType;

pub struct OutDataType {
    pub format_type: FormatType,
    pub(crate) buff_write: Option<Stdout>,
}

impl WritableData{
    fn to_camt053(&self, buff_write: Stdout) -> Result<(), ConvertError> {
        let mut writer = buff_write.lock();
        serde_xml_rs::to_writer(&mut writer, &self.camt_mt)?;
        Ok(())
    }

    fn mt940_field_6x(record_camt: &BkToCstmAttribute, record_write: &mut String) {
        for balance in  &record_camt.stmt.bal {
            match balance.tp.cd_or_party.cd.as_ref() {
                "OPBD" => {record_write.push_str(":60F:")},
                "CLBD" => {record_write.push_str(":62F:")},
                "CLAV" => {record_write.push_str(":62M:")},
                "ITAV" => {record_write.push_str(":64:")},
                "FPAV" => {record_write.push_str(":65:")},
                _ => {}
            }
            record_write.push_str(balance.tp.cd_or_party.cd.as_ref());
            let dt = balance.dt.dt.replace("-", "");
            if dt.len() >= 10 {
                record_write.push_str(&dt[2.. 10]);
            }
            record_write.push_str(balance.ccy.as_ref());
            record_write.push_str(balance.amt.replace(".", ",").as_ref());
            record_write.push_str("\n");
        }
    }

    fn mt940_field_61_86(record_camt: &Vec<NtryAttribute>, record_write: &mut String) {
        for ntry in record_camt {
            record_write.push_str(":61:");
            let mut dt = ntry.val_dt.dt.replace("-", "");
            if dt.len() >= 10 {
                record_write.push_str(&dt[2.. 10]);
            }
            dt = ntry.book_dt.dt.replace("-", "");
            if dt.len() >= 10 {
                record_write.push_str(&dt[4.. 10]);
            }
            if ntry.cdt_dbt_ind == "CRDT" {
                record_write.push_str(":C")}
            else {record_write.push_str(":D")};
            record_write.push_str(ntry.amt.replace(".", ",").as_ref());
            record_write.push_str(ntry.bx_tx_cd.prtry.cd.as_ref());
            if !ntry.ntry_dtls.btch.tx_dtls.is_empty() {
                record_write.push_str(ntry.ntry_dtls.btch.tx_dtls[0].refs.end_to_end_id.as_ref());
            }
            for tx_dtls in &ntry.ntry_dtls.btch.tx_dtls {
                record_write.push_str(":86:/NREF/");
                record_write.push_str(tx_dtls.refs.end_to_end_id.as_ref());
                record_write.push_str("\n");
                if !tx_dtls.rltd_pties.cdtr.nm.is_empty() {
                    record_write.push_str("/CRNM/");
                    record_write.push_str(tx_dtls.rltd_pties.cdtr.nm.as_ref());
                    record_write.push_str("\n");
                }
                if !tx_dtls.rltd_pties.cdtr_acct.other.id.is_empty() {
                    record_write.push_str("/CACT/");
                    record_write.push_str(tx_dtls.rltd_pties.cdtr_acct.other.id.as_ref());
                    record_write.push_str("\n");
                }
                if !tx_dtls.rltd_agts.cdtr_agt.bic.is_empty() {
                    record_write.push_str("/CBIC/");
                    record_write.push_str(tx_dtls.rltd_agts.cdtr_agt.bic.as_ref());
                    record_write.push_str("\n");
                }
                if !tx_dtls.rmt_inf.ustrd.is_empty() {
                    record_write.push_str("/REMI/");
                    record_write.push_str(tx_dtls.rmt_inf.ustrd.as_ref());
                    record_write.push_str("\n");
                }
                if !tx_dtls.addtl_tx_inf.is_empty() {
                    record_write.push_str("/OPRP/");
                    record_write.push_str(tx_dtls.addtl_tx_inf.as_ref());
                    record_write.push_str("\n");
                }
                if !tx_dtls.rltd_pties.dbtr_acct.other.id.is_empty() {
                    record_write.push_str("/DACT/");
                    record_write.push_str(tx_dtls.rltd_pties.dbtr_acct.other.id.as_ref());
                    record_write.push_str("\n");
                }
                if !tx_dtls.amt_dtls.amt.is_empty(){
                    record_write.push_str("/OAMT/");
                    record_write.push_str(tx_dtls.amt_dtls.amt.as_ref());
                    record_write.push_str("\n");
                }
                if !tx_dtls.rltd_pties.dbtr.id.othr.id.is_empty(){
                    record_write.push_str("/DCID/");
                    record_write.push_str(tx_dtls.rltd_pties.dbtr.id.othr.id.as_ref());
                    record_write.push_str("\n");
                }
            }
        }
    }
    fn to_mt940(&self, buff_write: Stdout) -> Result<(), ConvertError> {
        let mut record_write = String::new();
        let mut writer = buff_write.lock();
        for record in &self.camt_mt.bk_to_cstm{
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
            WritableData::mt940_field_6x(&record, &mut record_write);
            WritableData::mt940_field_61_86(&record.stmt.ntry, &mut record_write);
            record_write.push_str("}\n{5:-}\n");
            writer.write_all(record_write.as_bytes())?;
            writer.flush()?;
            record_write.clear();
        }
        Ok(())
    }

    fn to_csv(&self, buff_write: Stdout) -> Result<(), ConvertError> {
        Ok(())
    }
}

impl Document {
    pub(crate) fn to(document: DocumentType, out: OutDataType) -> Result<(), ConvertError> {
        let mut wr_data = WritableData { camt_mt: DocumentCamt053::default() };

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

