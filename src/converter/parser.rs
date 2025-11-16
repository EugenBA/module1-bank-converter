use crate::models::camt053::DocumentCamt053;
use crate::models::csv::DocumentCsv;
use crate::models::mt940::DocumentMt940;

impl From<DocumentCamt053>  for DocumentMt940 {
    fn from(camt053: DocumentCamt053) -> Self {
        Self {
            document: camt053.bk_to_cstm
        }
    }
}

impl From<DocumentCsv> for DocumentMt940 {
    fn from(csv: DocumentCsv) -> Self {
        let camt = DocumentCsv::parse_to_camt(csv);
        match camt {
            Ok(camt053) => DocumentMt940::from(camt053),
            Err(e) => panic!("Error convert csv to camt053: {}", e)
        }
    }
}

impl From<DocumentMt940> for DocumentCamt053 {
    fn from(mt940: DocumentMt940) -> Self {
        let mut camt = Self::default();
        camt.bk_to_cstm = mt940.document;
        camt
    }
}

impl From<DocumentCsv> for DocumentCamt053 {
    fn from(csv: DocumentCsv) -> Self {
        match DocumentCsv::parse_to_camt(csv) {
            Ok(camt053) => camt053,
            Err(e) => panic!("Error convert csv to camt053: {}", e)
        }
    }
}

impl From<DocumentCamt053> for DocumentCsv {
    fn from(camt053: DocumentCamt053) -> Self {
        match DocumentCsv::parse_to_csv(&camt053) {
            Ok(csv) => csv,
            Err(e) => { panic!("Error convert camt053 to csv: {}", e);}
        }
    }
}

impl From<DocumentMt940> for DocumentCsv {
    fn from(mt940: DocumentMt940) -> Self {
        let camt = DocumentCamt053::from(mt940);
        match DocumentCsv::parse_to_csv(&camt) {
            Ok(csv) => csv,
            Err(e) => { panic!("Error convert camt053 to csv: {}", e);}
        }
    }
}

/*
impl <T:Read>  Default for PipelineConverter<T> {
    fn default() -> Self {
        PipelineConverter {
            data_in: InputDataType {
                format_type: FormatType::None,
                buff_read: None
            },
            data_out: OutDataType {
                format_type: FormatType::None,
                buff_write: None
            }
        }
    }
}

impl<T:Read>  PipelineConverter<T> {
    pub fn get_format_type_from_string(format_str: &String) -> FormatType {
        match format_str.to_lowercase().as_str() {
            "csv" => FormatType::Csv,
            "xml" => FormatType::Xml,
            "mt940" => FormatType::Mt940,
            "camt053" => FormatType::Camt053,
            _=> FormatType::None
        }
    }
    
    pub fn convert_file(pipeline: PipelineConverter<T>) -> Result<(), ConvertError> {
        if pipeline.data_in.format_type == FormatType::None {
            return Err(ConvertError::BadArgument("Not support input format".to_string()));
        }
        if pipeline.data_out.format_type == FormatType::None {
            return Err(ConvertError::BadArgument("Not support output format".to_string()));
        }
        let document = Document::from(pipeline.data_in).parse_result?;
        Document::to(document, pipeline.data_out)?;
        Ok(())
    }
}
*/
