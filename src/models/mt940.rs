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