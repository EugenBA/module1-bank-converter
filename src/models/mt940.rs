use crate::models::camt053::{BkToCstmAttribute};
pub struct DocumentMt940 {
    document: Vec<BkToCstmAttribute>
}

impl DocumentMt940 {
    pub fn default() -> Self {
        Self{
            document: Vec::new()
        }
    }
    
}

pub(crate) enum DocumentMt940H2Direction{
    Input,
    Output,
    UNKNOW
}

pub(crate) struct DocumentMt940H2{
    pub(crate) direction: DocumentMt940H2Direction,
    pub(crate) reference: String
}

impl DocumentMt940H2{
    pub(crate) fn default() -> Self{
        Self{
            direction: DocumentMt940H2Direction::UNKNOW,
            reference: "UNKNOWN_REFERENCE".to_string(),
        }
    }
}