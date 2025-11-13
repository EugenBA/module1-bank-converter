use crate::errors::{ConvertError};
use crate::converter::reader::{Document, InputDataType};

#[derive(PartialEq)]
pub enum FormatType {
    None,
    Csv,
    Xml,
    Mt940,
    Camt053,
}

pub struct PipelineConverter {
    pub data_in: InputDataType,
    pub data_out: InOutData
}

pub struct InOutData{
    pub file_name: String,
    pub format_type: FormatType
}

impl Default for InOutData {
    fn default() -> InOutData {
        InOutData{ file_name: "".to_string(), format_type: FormatType::None }
    }
}
impl Default for PipelineConverter {
    fn default() -> Self {
        PipelineConverter { data_in: InputDataType{
            format_type: FormatType::None,
            buff_read: None
        },
            data_out: InOutData::default() }
    }
}

impl PipelineConverter {
    pub fn get_format_type_from_string(format_str: &String) -> FormatType {
        match format_str.to_lowercase().as_str() {
            "csv" => FormatType::Csv,
            "xml" => FormatType::Xml,
            "mt940" => FormatType::Mt940,
            "camt053" => FormatType::Camt053,
            _=> FormatType::None
        }
    }
    
    pub fn convert_file(pipeline: PipelineConverter) -> Result<(), ConvertError> {
        if pipeline.data_in.format_type == FormatType::None {
            return Err(ConvertError::BadArgument("Not support input format".to_string()));
        }
        if pipeline.data_out.format_type == FormatType::None {
            return Err(ConvertError::BadArgument("Not support output format".to_string()));
        }
        let document = Document::from(pipeline.data_in).parse_result?;
        Ok(())
    }
}