use serde::Serialize;
use serde::Deserialize;
use crate::csv_data;

pub struct DocumentCsv {
    rows: Vec<RowCsv>
}
csv_data!(RowCsv, String, a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u);
