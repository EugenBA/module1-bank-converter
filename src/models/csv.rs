use regex::Regex;
use serde::Serialize;
use serde::Deserialize;
use crate::csv_data;
use crate::errors::ParserError;
use crate::models::camt053::{BalanceAttribute, BkToCstmAttribute, DocumentCamt053,
                             NtryAttribute, TxDtlsAttribute};

pub struct DocumentCsv {
    pub(crate) rows: Vec<RowCsv>
}
csv_data!(RowCsv, String, a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u);


impl DocumentCsv {
    pub(crate) fn new() -> Self{
        Self{rows: Vec::new()}
    }

    fn extract_time_create(val: &str) -> Option<String>{
        let reg_pattern = Regex::new(r"(\d{2}:\d{2}:\d{2})");
        if let Ok(regexp) = reg_pattern {
            if let Some(capture) = regexp.captures(val)
            {
                let dt = capture[1].to_string();
                return Some(dt);
            }
        }
        None
    }
    fn extract_date_create(val: &str) -> Option<String> {
        let reg_pattern = Regex::new(r"(\d{2}.\d{2}.\d{4})");
        if let Ok(regexp) = reg_pattern {
            if let Some(capture) = regexp.captures(val)
            {
                let mut dt = String::new();
                dt.push_str(&capture[1][6..10]);
                dt.push_str("-");
                dt.push_str(&capture[1][3..5]);
                dt.push_str("-");
                dt.push_str(&capture[1][0..2]);
                return Some(dt);
            }
        }
        None
    }

    fn convert_ru_month_to_number(val: &str) -> Option<String> {
        let number_month = val.replace("января", "01")
            .replace("февраля", "02")
            .replace("марта", "03")
            .replace("апреля", "04")
            .replace("мая", "05")
            .replace("июня", "06")
            .replace("июля","07")
            .replace("августа", "08")
            .replace("сентября", "09")
            .replace("октября", "10")
            .replace("ноября", "11")
            .replace("декабря", "12");
        if number_month.len() == 2 {
            return Some(number_month);
        }
        None
    }

    fn extract_date_rus(val: &str) -> Option<String> {
        let reg_pattern = Regex::new(r"(\d{2}).(января|февраля|марта|апреля|мая|июня|июля|августа|сентября|октября|ноября|декабря).(\d{4})");
        if let Ok(regexp) = reg_pattern {
            if let Some(capture) = regexp.captures(val)
            {
                let mut dt = String::new();
                dt.push_str(&capture[3]);
                dt.push_str("-");
                if let Some(month) = Self::convert_ru_month_to_number(&capture[2]){
                    dt.push_str(&month);
                }
                dt.push_str("-");
                dt.push_str(&capture[1]);
                return Some(dt);
            }
        }
        None
    }

    fn extract_crd_agent(val: &str, ntry_det: &mut TxDtlsAttribute) {
        let reg_pattern = Regex::new(r"(\d+) ([\w ]+), (.+)");
        if let Ok(regexp) = reg_pattern {
            if let Some(capture) = regexp.captures(val)
            {
                ntry_det.rltd_agts.dbtr_agt.bic = capture[1].to_string();
                ntry_det.rltd_agts.dbtr_agt.nm = capture[2].to_string();
            }
        }
    }

    fn extract_ccy(val: &str) -> Option<String>{
        let ccy = val.replace("Российский рубль", "RUB")
            .replace("Доллар США", "USD")
            .replace("Евро", "EUR");
        if ccy.len() == 3 {
            return Some(ccy);
        }
        None
    }

    pub(crate) fn parse_to_camt(self) -> Result<DocumentCamt053, ParserError>{
        let mut camt = DocumentCamt053::new();
        let mut camt_bk_to_cstm = BkToCstmAttribute::default();
        if self.rows.len() < 8 {
            return Err(ParserError::BadInputFormatFile("Bad input csv file".to_string()))
        }
        if let Some(date_create) = DocumentCsv::extract_date_create(&self.rows[3].b){
            camt_bk_to_cstm.grp_hdr.crd_dt_tm = date_create;
            if let Some(time_create) = DocumentCsv::extract_time_create(&self.rows[3].b){
                camt_bk_to_cstm.grp_hdr.crd_dt_tm.push_str("T");
                camt_bk_to_cstm.grp_hdr.crd_dt_tm.push_str(&time_create);
            }
        }
        camt_bk_to_cstm.stmt.acct.id.othr.id = self.rows[4].m.to_string();
        camt_bk_to_cstm.stmt.acct.ownr.nm = self.rows[5].m.to_string();
        if let Some(dt) = DocumentCsv::convert_ru_month_to_number(&self.rows[6].c) {
            camt_bk_to_cstm.stmt.fr_to_dt.fr_dt_tm = dt + "T00:00:00";
        }
        if let Some(dt) = DocumentCsv::convert_ru_month_to_number(&self.rows[6].p){
            camt_bk_to_cstm.stmt.fr_to_dt.to_dt_tm= dt + "T23:59:59";
        }
        if let Some(ccy) = DocumentCsv::extract_ccy(&self.rows[7].c){
            camt_bk_to_cstm.stmt.acct.ccy =ccy;
        }
        for index_row in 9..self.rows.len() {
            let row = &self.rows[index_row];
            if row.b.is_empty() {
                break;
            }
            let mut ntry = NtryAttribute::default();
            let mut ntry_det = TxDtlsAttribute::default();
            ntry.ccy = camt_bk_to_cstm.stmt.acct.ccy.clone();
            if row.j.is_empty(){
                ntry.cdt_dbt_ind = "CDIT".to_string();
                ntry.amt = row.n.to_string();
            }
            else{
                ntry.cdt_dbt_ind =  "DBIT".to_string();
                ntry.amt = row.j.to_string();
            }
            ntry.bx_tx_cd.prtry.cd = row.q.to_string();
            ntry.bx_tx_cd.prtry.issr = self.rows[2].b.to_string();
            ntry.acct_svrc_ref = row.o.to_string();
            ntry_det.refs.end_to_end_id ="1".to_string();
            let debit_detals: Vec<&str> = row.e.split("\n").collect();
            if debit_detals.len() == 3{
                ntry_det.rltd_pties.dbtr.nm = debit_detals[2].to_string();
                ntry_det.rltd_pties.dbtr.id.othr.id = debit_detals[1].to_string();
                ntry_det.rltd_pties.dbtr_acct.other.id = debit_detals[0].to_string();
            }
            let credit_detals: Vec<&str> = row.i.split("\n").collect();
            if credit_detals.len() == 3{
                ntry_det.rltd_pties.cdtr.nm = debit_detals[2].to_string();
                ntry_det.rltd_pties.cdtr.id.othr.id = debit_detals[1].to_string();
                ntry_det.rltd_pties.cdtr_acct.other.id = debit_detals[0].to_string();
            }
            DocumentCsv::extract_crd_agent(&row.r, & mut ntry_det);
            ntry_det.rmt_inf.ustrd = row.u.to_string();
            ntry.ntry_dtls.btch.tx_dtls.push(ntry_det);
        }
        let next_row = self.rows.len() - 4; //offset balance data from end document
        let mut balance_opbd = BalanceAttribute::default();
        balance_opbd.ccy = camt_bk_to_cstm.stmt.acct.ccy.clone();
        balance_opbd.tp.cd_or_party.cd = "OPDB".to_string();
        balance_opbd.amt = self.rows[next_row+1].h.to_string();
        camt_bk_to_cstm.stmt.bal.push(balance_opbd);
        camt_bk_to_cstm.stmt.txs_summry.ttl_dbt_ntries.sum = self.rows[next_row+2].h.to_string();
        camt_bk_to_cstm.stmt.txs_summry.ttl_cdt_ntries.sum = self.rows[next_row+2].l.to_string();
        camt_bk_to_cstm.stmt.txs_summry.ttl_ntries.nb_of_ntries = self.rows[next_row].l.to_string();
        let mut balance_clbd = BalanceAttribute::default();
        balance_clbd.ccy = camt_bk_to_cstm.stmt.acct.ccy.clone();
        balance_clbd.tp.cd_or_party.cd = "CLDB".to_string();
        balance_clbd.amt = self.rows[next_row+3].l.to_string();
        camt_bk_to_cstm.stmt.bal.push(balance_clbd);
        camt.bk_to_cstm.push(camt_bk_to_cstm);
        Ok(camt)
    }

    pub(crate)  fn parse_to_csv(camt: &DocumentCamt053) -> Result<Self, ParserError>{
        let csv = DocumentCsv::new();
        Ok(csv)
    }
}
