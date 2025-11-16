use serde::Serialize;
use serde::Deserialize;
use crate::csv_data;
use crate::errors::ParserError;
use crate::models::camt053::DocumentCamt053;

pub struct DocumentCsv {
    pub(crate) rows: Vec<RowCsv>
}
csv_data!(RowCsv, String, a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u);

impl DocumentCsv {
    pub(crate) fn new() -> Self{
        Self{rows: Vec::new()}
    }
    
    pub(crate) fn parse_to_camt(self) -> Result<DocumentCamt053, ParserError>{
        let camt = DocumentCamt053::new();
        for row in self.rows {
          //  camt.add_transaction(row);
        }
        Ok(camt)
    }
    
    pub(crate)  fn parse_to_csv(camt: &DocumentCamt053) -> Result<Self, ParserError>{
        let csv = DocumentCsv::new();
        Ok(csv)
    }
}
