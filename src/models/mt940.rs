use regex::Regex;
use crate::models::camt053::{BalanceAttribute, BkToCstmrStmt, DtAttribute, NtryAttribute, NtryDtlsAttribute, TxDtlsAttribute};

pub struct DocumentMt940 {
    pub(crate) document: Vec<BkToCstmrStmt>
}

impl DocumentMt940 {

    pub(crate) fn find_record(document: &str) -> Option<Vec<(usize, usize)>> {
        let mut vec_start_pattern: Vec<usize> = Vec::new();
        let mut vec_end_pattern: Vec<usize> = Vec::new();
        let mut records: Vec<(usize, usize)> = Vec::new();
        for (index, _found_pattern) in document.match_indices("{1"){
            vec_start_pattern.push(index);
        }
        for (index, _found_pattern) in document.match_indices("{5:"){
            vec_end_pattern.push(index);
        }
        for i in vec_start_pattern.iter().enumerate(){
            records.push((vec_start_pattern[i.0], vec_end_pattern[i.0]));
        }
        Some(records)
    }

    fn parse_field_one(header: &str) -> String{
        let regex = Regex::new(r"F\d{2}([A-Z]*\d*[A-Z]*)\d").unwrap();
        if let Some(capture) = regex.captures(header) {
            capture[1].to_string()
        }
        else {
            "UNKNOW_BIC".to_string()
        }
    }

    fn parse_field_two(header: &str, document: &mut BkToCstmrStmt) {
        let regex = Regex::new(r"([IO])(\d{3})(.*?)");
        if let Ok(regex) = regex {
            if let Some(capture) = regex.captures(header) {
                document.grp_hdr.msg_id = capture[3].to_string();
                document.stmt.id = capture[3].to_string() + "-940";
            }
        }
    }

    fn parse_field_balance(header: &str) -> Option<BalanceAttribute>{
        let regex = Regex::new(r"([CD])(\d{6})([A-Z]+)(\d+,\d+)");
        if let Ok(regex) = regex {
            if let Some(capture) = regex.captures(header) {
                let mut balance = BalanceAttribute::default();
                balance.dt= DtAttribute::format_dt(&capture[2]);
                balance.amt.ccy = capture[3].to_string();
                balance.amt.amt = capture[4].replace(",", ".").to_string();
                balance.cd = capture[1].to_string();
                return Some(balance);
            }
        }
        None
    }
    fn parse_field_61(field: &str, vault: &str, ntry: &mut NtryAttribute){
        let regex = Regex::new(r"(\d{6})(\d{4})([CD])(\d+,\d+)([A-Z]{4})(\w+)");
        if let Ok(regex) = regex {
            if let Some(capture) = regex.captures(field) {
                ntry.val_dt = DtAttribute::format_dt(&capture[1]);
                let dt =  capture[1][0 .. 2].to_string() + &capture[2][0..4].to_string();
                ntry.bookg_dt = DtAttribute::format_dt(&dt);
                ntry.bx_tx_cd.prtry.cd = capture[5].to_string();
                ntry.amt = capture[4].replace(",", ".").to_string();
                ntry.ccy = vault.to_string();
                ntry.cdt_dbt_ind  = if capture[3].to_string() == "C".to_string(){
                    "CRDT".to_string()
                } else { "DBIT".to_string()};
                let mut nxdet: NtryDtlsAttribute = NtryDtlsAttribute::default();
                DocumentMt940::parse_field_86(field, &mut nxdet);
                let mut tlds: TxDtlsAttribute = TxDtlsAttribute::default();
                tlds.refs.end_to_end_id = capture[6].to_string();
                nxdet.btch.tx_dtls.push(tlds);
            }
        }
    }

    fn parse_field_86(field: &str, ntrydet: &mut NtryDtlsAttribute) {
        let reg_pattern = Regex::new(r"/([A-Z]{4})/([^/]*)");
        if let Ok(regexp) = reg_pattern {
            let mut tlds: TxDtlsAttribute = TxDtlsAttribute::default();
            for capture in regexp.captures_iter(field){
                match &capture[1] {
                    "EREF" =>{
                        tlds.refs.end_to_end_id = capture[2].to_string();
                    },
                    "CRNM" =>{
                        tlds.rltd_pties.cdtr.nm = capture[2].to_string();
                    },
                    "CACT" => {
                        tlds.rltd_pties.cdtr_acct.other.id = capture[2].to_string();
                    },
                    "CBIC" => {
                        tlds.rltd_agts.cdtr_agt.bic = capture[2].to_string();
                    },
                    "REMI" =>{
                        tlds.rmt_inf.ustrd = capture[2].to_string();
                    },
                    "OPRP" =>{
                        tlds.addtl_tx_inf = capture[2].to_string();
                    },
                    "DACT" =>{
                        tlds.rltd_pties.dbtr_acct.other.id = capture[2].to_string();
                    },
                    "DBIC" =>{
                        tlds.rltd_pties.dbtr_acct.other.id = capture[2].to_string();
                    },
                    "OAMT" =>{
                        tlds.amt_dtls.amt = capture[2].to_string();
                    },
                    "DCID" =>{
                        tlds.rltd_pties.dbtr.id.othr.id = capture[2].to_string();
                    },
                    _ => {}
                }
            }
            ntrydet.btch.tx_dtls.push(tlds);
        }
    }

    fn parse_field_ntry(header: &str, vault: &str) -> Option<Vec<NtryAttribute>>{
        let mut reg_pattern = Regex::new(r":61:([\n\w\d ,/-]+):");
        let mut field_61: Vec<String> = Vec::new();
        let mut field_86: Vec<String> = Vec::new();
        let mut nxtry : Vec<NtryAttribute> = Vec::new();
        if let Ok(regexp) = reg_pattern {
            for capture in regexp.captures_iter(header){
                field_61.push(capture[1].to_string());
            }
        }
        reg_pattern = Regex::new(r":86:([\n\w\d ,/-]+):");
        if let Ok(regexp) = reg_pattern {
            for capture in regexp.captures_iter(header){
                field_86.push(capture[1].to_string());
            }
        }
        let unions: Vec<(String, String)> = field_61.into_iter().zip(field_86.into_iter()).collect();
        for union in unions.iter(){
            let mut ntry = NtryAttribute::default();
            DocumentMt940::parse_field_61(&union.0, vault, &mut ntry);
            nxtry.push(ntry);
        }
        Some(nxtry)
    }

    fn parse_field_foo(header: &str, document: &mut BkToCstmrStmt) {
        let reg_codes = ["26", "25", "28C", "60F", "62F", "62M", "64", "65"];
        for reg_code in reg_codes.iter() {
            let reg_pattern = Regex::new(&format!(r":{}:([\n\w\d ,/-]+):",
                                                  reg_code));
            if let Ok(regexp) = reg_pattern{
                if let Some(capture) = regexp.captures(header){
                    match reg_code.as_ref() {
                        "26" => {
                            document.grp_hdr.msg_id = capture[1].to_string();
                            document.stmt.id = capture[1].to_string();
                        },
                        "25" => {
                            document.stmt.acct.ownr.id.org_id.othr.id = capture[1].to_string();
                        },
                        "28C" => {
                            let fields: Vec<&str> = capture[1].split('/').collect();
                            if fields.len() > 1{
                                document.stmt.elctrnc_seq_nb = fields[0].to_string();
                                document.stmt.lgl_seq_nv = fields[1].to_string();
                            }
                        },
                        "60F" | "62F" | "62M" | "64"  | "65"=> {
                            if  let Some(mut balance) = DocumentMt940::parse_field_balance(&capture[1]){
                                if *reg_code == "60F" {
                                    balance.tp.cd_or_party.cd = "OPBD".to_string();
                                }
                                if *reg_code == "62F" {
                                    balance.tp.cd_or_party.cd = "CLBD".to_string();
                                }
                                if *reg_code == "62M" {
                                    balance.tp.cd_or_party.cd = "CLAV".to_string();
                                }
                                if *reg_code == "64" {
                                    balance.tp.cd_or_party.cd = "ITAV".to_string();
                                }
                                if *reg_code == "65" {
                                    balance.tp.cd_or_party.cd = "FPAV".to_string();
                                }
                                document.stmt.bal.push(balance);
                            }
                        },
                        _=>{}
                    }
                }
            }
        }
        if let Some(ntry) = DocumentMt940::parse_field_ntry(&header,
                                                            &document.stmt.bal[0].amt.ccy){
            document.stmt.ntry = ntry;
        }//todo: replace this only test
    }

    pub(crate) fn parse_one_record(document: &str) -> Option<BkToCstmrStmt> {
        let mut record: BkToCstmrStmt = BkToCstmrStmt::default();
        for field in 1..6 {
            let reg_pattern = Regex::new(&format!(r"\{{{}:[\n\w\d ,/:-]*\}}",
                                                  field));
            if let Ok(regexp) = reg_pattern {
                match regexp.captures(document) {
                    Some(capture) => {
                        match field {
                            1 => { record.stmt.acct.svcr.fin_instn_id.bic =
                                DocumentMt940::parse_field_one(&capture[1].to_string());},
                            2 => {
                                DocumentMt940::parse_field_two(&capture[1].to_string(),
                                                               &mut record);
                            },
                            4 => { DocumentMt940::parse_field_foo(&capture[1].to_string(),
                                                                  &mut record); },
                            _ => {}
                        }
                    }
                    None => {}
                }
            }
        }
        Some(record)
    }
    pub(crate) fn extract_field_6x_mt940(record_camt: &BkToCstmrStmt, record_write: &mut String) {
        for balance in &record_camt.stmt.bal {
            match balance.tp.cd_or_party.cd.as_ref() {
                "OPBD" => { record_write.push_str(":60F:") },
                "CLBD" => { record_write.push_str(":62F:") },
                "CLAV" => { record_write.push_str(":62M:") },
                "ITAV" => { record_write.push_str(":64:") },
                "FPAV" => { record_write.push_str(":65:") },
                _ => {}
            }
            record_write.push_str(balance.tp.cd_or_party.cd.as_ref());
            let dt = balance.dt.dt.replace("-", "");
            if dt.len() >= 10 {
                record_write.push_str(&dt[2..10]);
            }
            record_write.push_str(balance.amt.ccy.as_ref());
            record_write.push_str(balance.amt.amt.replace(".", ",").as_ref());
            record_write.push_str("\n");
        }
    }

    pub(crate) fn extract_field_61_86_mt940(record_camt: &Vec<NtryAttribute>, record_write: &mut String) {
        for ntry in record_camt {
            record_write.push_str(":61:");
            let mut dt = ntry.val_dt.dt.replace("-", "");
            if dt.len() >= 10 {
                record_write.push_str(&dt[2..10]);
            }
            dt = ntry.bookg_dt.dt.replace("-", "");
            if dt.len() >= 10 {
                record_write.push_str(&dt[4..10]);
            }
            if ntry.cdt_dbt_ind == "CRDT" {
                record_write.push_str(":C")
            } else { record_write.push_str(":D") };
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
                if !tx_dtls.amt_dtls.amt.is_empty() {
                    record_write.push_str("/OAMT/");
                    record_write.push_str(tx_dtls.amt_dtls.amt.as_ref());
                    record_write.push_str("\n");
                }
                if !tx_dtls.rltd_pties.dbtr.id.othr.id.is_empty() {
                    record_write.push_str("/DCID/");
                    record_write.push_str(tx_dtls.rltd_pties.dbtr.id.othr.id.as_ref());
                    record_write.push_str("\n");
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::camt053::{HeaderAttribute, StatementAttribute};
    use super::*;

    #[test]
    fn test_find_record(){
        let doc ="{1:}{5:-}{1:   }{2:}{3:}{4:}{5:-}".to_string();
        let result: Vec<(usize, usize)> = vec![(0, 4), (9, 28)];
        assert_eq!(DocumentMt940::find_record(&doc).unwrap(), result);
    }
    #[test]
    fn test_parse_field_one() {
        let doc = "F01ASNBNL21XXXX0000000000".to_string();
        assert_eq!(DocumentMt940::parse_field_one(&doc), "ASNBNL21XXXX".to_string());
    }

    #[test]
    fn test_parse_field_two() {
        let doc = "{2:O940ASNBNL21XXXXN}".to_string();
        let mut result = BkToCstmrStmt::default();
        DocumentMt940::parse_field_two(&doc, &mut result);
        assert_eq!(result, BkToCstmrStmt {
            grp_hdr: HeaderAttribute {
                msg_id: "ASNBNL21XXXXN".to_string(),
                cre_dt_tm: "".to_string(),
            },
            stmt: StatementAttribute {
                id: "ASNBNL21XXXXN-940".to_string(),
                elctrnc_seq_nb: "".to_string(),
                lgl_seq_nv: "".to_string(),
                cre_dt_tm: "".to_string(),
                fr_to_dt: Default::default(),
                acct: Default::default(),
                bal: vec![],
                txs_summry: Default::default(),
                ntry: vec![],
            },
        });
    }
}
