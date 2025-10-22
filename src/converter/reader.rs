use crate::converter::parser::FormatType;
pub struct PipelineParse {
    pub data_in: InOutData,
    pub data_out: InOutData
}

pub struct InOutData{
    pub file_name: String,
    pub format_type: FormatType
}

impl PipelineParse {
    pub fn get_format_type_from_string(format_str: &String) -> FormatType{
        match format_str.to_lowercase().as_str() {
            "csv" => FormatType::Csv,
            "xml" => FormatType::Xml,
            "mt940" => FormatType::Mt940,
            "camt053" => FormatType::Camt053,
            _=> FormatType::None
        }
    }
}
impl Default for InOutData {
    fn default() -> InOutData {
        InOutData{ file_name: "".to_string(), format_type: FormatType::None }
    }
}
impl Default for PipelineParse {
    fn default() -> Self {
        PipelineParse { data_in: InOutData::default(), data_out: InOutData::default() }
    }
}
