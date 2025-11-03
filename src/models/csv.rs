use serde::Serialize;
use serde::Deserialize;
use crate::csv_data;

pub struct DocumentCsv {
    rows: Vec<RowCsv>
}
csv_data! (RowCsv, String, f1, f2, f3, f4, f5, f6, f7, f8, f9, f10, f11, f12,
                                f13, f14, f15, f16, f17, f18, f19, f20, f21);
