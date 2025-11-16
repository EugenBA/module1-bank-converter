use regex::Regex;
use serde::Serialize;
use serde::Deserialize;
use crate::csv_data;
use crate::errors::ParserError;
use crate::models::camt053::{BkToCstmAttribute, DocumentCamt053};

pub struct DocumentCsv {
    pub(crate) rows: Vec<RowCsv>
}
csv_data!(RowCsv, String, a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u);

impl DocumentCsv {
    pub(crate) fn new() -> Self{
        Self{rows: Vec::new()}
    }

    fn extract_date_create(val: &str) -> Option<String> {
        let reg_pattern = Regex::new(r"(\d{2}.\d{2}.\d{4})\W\w\W(\d{2}:\d{2}:\d{2})");
        if let Ok(regexp) = reg_pattern {
            if let Some(capture) = regexp.captures(val)
            {
                let mut dt = String::new();
                dt.push_str(&capture[1][6..10]);
                dt.push_str("-");
                dt.push_str(&capture[1][3..5]);
                dt.push_str("-");
                dt.push_str(&capture[1][0..2]);
                dt.push_str("T");
                dt.push_str(&capture[2]);
                return Some(dt);
            }
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
        }
        camt_bk_to_cstm.stmt.acct.id.othr.id = self.rows[4].m.to_string();
        camt_bk_to_cstm.stmt.acct.ownr.nm = self.rows[5].m.to_string();
        for row in self.rows {
          //  camt.add_transaction(row);
        }
        camt.bk_to_cstm.push(camt_bk_to_cstm);
        Ok(camt)
    }

    pub(crate)  fn parse_to_csv(camt: &DocumentCamt053) -> Result<Self, ParserError>{
        let csv = DocumentCsv::new();
        Ok(csv)
    }
}
