use std::io::{BufRead, Stdin};
use crate::converter::parser::FormatType;
use crate::errors::ParserError;
use crate::models::camt053::{BalanceAttribute, BkToCstmAttribute, DocumentCamt053, DtAttribute, NtryAttribute};
use crate::models::mt940::{DocumentMt940};
use crate::models::csv::{DocumentCsv, RowCsv};
use csv::Reader;
use regex::{Regex};


pub struct InputDataType {
    format_type: FormatType,
    buff_read: Stdin,
}

enum Document{
    DocumentCamt053(Result<DocumentCamt053, ParserError>),
    DocumentMt940(Result<DocumentMt940, ParserError>),
    DocumentCsv(Result<DocumentCsv, ParserError>),
    None(ParserError)
}

impl From<InputDataType> for Document {
    fn from(value: InputDataType) -> Self {
        match value.format_type {
            FormatType::Camt053 | FormatType::Xml => {
                Self::DocumentCamt053(DocumentCamt053::from_camt053(value.buff_read))
            },
            FormatType::Mt940 => {
                Self::DocumentMt940(DocumentMt940::from_mt940(value.buff_read))
            }
            FormatType::Csv => {
                Self::DocumentCsv(DocumentCsv::from_csv(value.buff_read))
            },
            _ => { Self::None(ParserError::BadInputFormatFile("Bad input type file".to_string())) }
        }
    }
}

impl DocumentCamt053 {
    pub fn new() -> Self {
        DocumentCamt053::default()
    }
    
    fn from_camt053(buf_read: Stdin) -> Result<Self, ParserError> {
        let mut reader = buf_read.lock();
        let mut xml_str = String::new();
        while let Ok(byte_reader) = reader.read_line(&mut xml_str) {
            if byte_reader == 0 {
                break;
            }
        }
        serde_xml_rs::from_str(&xml_str)?
    }

}

impl DocumentMt940 {

    fn find_record(document: &str) -> Option<Vec<(usize, usize)>> {
        let mut vec_start_pattern: Vec<usize> = Vec::new();
        let mut vec_end_pattern: Vec<usize> = Vec::new();
        let mut records: Vec<(usize, usize)> = Vec::new();
        for (index, _found_pattern) in document.match_indices("{1"){
            vec_start_pattern.push(index);
        }
        for (index, _found_pattern) in document.match_indices("{5:"){
            vec_end_pattern.push(index);
        }
        for i in vec_start_pattern.iter().enumerate(){
            records.push((vec_start_pattern[i.0], vec_end_pattern[i.0]));
        }
        Some(records)
    }

    fn parse_field_one(header: &str) -> String{
        let regex = Regex::new(r"F\d{2}([A-Z]*\d*[A-Z]*)\d").unwrap();
        if let Some(capture) = regex.captures(header) {
            capture[1].to_string()
        }
        else {
            "UNKNOW_BIC".to_string()
        }
    }

    fn parse_field_two(header: &str, document: &mut BkToCstmAttribute) {
        let regex = Regex::new(r"([IO])(\d{3})(.*?)");
        if let Ok(regex) = regex {
            if let Some(capture) = regex.captures(header) {
                document.grp_hdr.msg_id = capture[2].to_string();
                document.stmt.id = capture[2].to_string() + "-940";
            }
        }
    }

    fn parse_field_balance(header: &str) -> Option<BalanceAttribute>{
        let regex = Regex::new(r"C(\d{6})([A-Z]+)(\d+,\d+)");
        if let Ok(regex) = regex {
            if let Some(capture) = regex.captures(header) {
                let mut balance = BalanceAttribute::default();
                balance.dt= DtAttribute::format_dt(&capture[1]);
                balance.ccy = capture[2].to_string();
                balance.amt = capture[3].replace(",", ".").to_string();
                return Some(balance);
            }
        }
        None
    }
    fn parse_field_61(field: &str, ntry: &mut NtryAttribute){
        let regex = Regex::new(r"(\d{6})(\d{4})([CD])(\d+,\d+)([A-Z]{4})(\w+)");
        if let Ok(regex) = regex {
            if let Some(capture) = regex.captures(field) {
                ntry.val_dt = DtAttribute::format_dt(&capture[1]);
                let dt =  capture[1][0 .. 2].to_string() + &capture[2][0..4].to_string();
                ntry.book_dt = DtAttribute::format_dt(&dt);
                ntry.amt = capture[4].replace(",", ".").to_string();
                ntry.cdt_dbt_ind  = if capture[3].to_string() == "C".to_string(){
                    "CRDT".to_string()
                } else { "DBIT".to_string()};
                //let mut nxdet = 
            }
        }
    }

    fn parse_field_86(field: &str, ntry: &mut NtryAttribute){
       //let regex = Regex::new(r":84:([\n\w\d ,/-]+):");
       // if let Ok(regex) = regex {
       //     if let Some(capture) = regex.captures(header) {
        //        ntry.amt = capture[1].to_string();
        //    }
        //}
    }

    fn parse_field_61_84(header: &str) -> Option<Vec<NtryAttribute>>{
        let mut reg_pattern = Regex::new(r":61:([\n\w\d ,/-]+):");
        let mut field_61: Vec<String> = Vec::new();
        let mut field_86: Vec<String> = Vec::new();
        let mut nxtry : Vec<NtryAttribute> = Vec::new();
        if let Ok(regexp) = reg_pattern {
            for capture in regexp.captures_iter(header){
                field_61.push(capture[1].to_string());
            }
        }
        reg_pattern = Regex::new(r":86:([\n\w\d ,/-]+):");
        if let Ok(regexp) = reg_pattern {
            for capture in regexp.captures_iter(header){
                field_86.push(capture[1].to_string());
            }
        }
        let unions: Vec<(String, String)> = field_61.into_iter().zip(field_86.into_iter()).collect();
        for union in unions.iter(){
            let mut ntry = NtryAttribute::default();
            DocumentMt940::parse_field_61(&union.0, &mut ntry);
            DocumentMt940::parse_field_86(&union.1, &mut ntry);
            nxtry.push(ntry);
        }
        Some(nxtry)
    }

    fn parse_field_foo(header: &str, document: &mut BkToCstmAttribute) {
        let reg_codes = ["26", "25", "28C", "60F", "62F", "62M", "64", "65"];
        for reg_code in reg_codes.iter() {
            let reg_pattern = Regex::new(&format!(r":{}:([\n\w\d ,/-]+):",
                                                  reg_code));
            if let Ok(regexp) = reg_pattern{
                if let Some(capture) = regexp.captures(header){
                    match reg_code.as_ref() {
                        "26" => {
                            document.grp_hdr.msg_id = capture[1].to_string();
                            document.stmt.id = capture[1].to_string();
                        },
                        "25" => {
                            document.stmt.acct.ownr.id.org_id.othr.id = capture[1].to_string();
                        },
                        "28C" => {
                            let fields: Vec<&str> = capture[1].split('/').collect();
                            if fields.len() > 1{
                                document.stmt.electr_seq_nb = fields[0].to_string();
                                document.stmt.lgl_seq_nv = fields[1].to_string();
                            }
                        },
                        "60F" | "62F" | "62M" | "64"  | "65"=> {
                            if  let Some(mut balance) = DocumentMt940::parse_field_balance(&capture[1]){
                                if *reg_code == "60F" {
                                    balance.tp.cd_or_party.cd = "OPBD".to_string();
                                }
                                if *reg_code == "62F" {
                                    balance.tp.cd_or_party.cd = "CLBD".to_string();
                                }
                                if *reg_code == "62M" {
                                    balance.tp.cd_or_party.cd = "CLAV".to_string();
                                }
                                if *reg_code == "64" {
                                    balance.tp.cd_or_party.cd = "ITAV".to_string();
                                }
                                if *reg_code == "60F" {
                                    balance.tp.cd_or_party.cd = "FPAV".to_string();
                                }
                                document.stmt.bal.push(balance);
                            }
                        },
                        _=>{}
                    }
                }
            }
        }
        if let Some(ntry) = DocumentMt940::parse_field_61_84(&header){
            document.stmt.ntry = ntry;
        }
    }

    fn parse_one_record(document: &str) -> Option<BkToCstmAttribute> {
        let mut record: BkToCstmAttribute = BkToCstmAttribute::default();
        for field in 1..6 {
            let reg_pattern = Regex::new(&format!(r"\{{{}:[\n\w\d ,/:-]*\}}",
                                                  field));
            if let Ok(regexp) = reg_pattern {
                match regexp.captures(document) {
                    Some(capture) => {
                        match field {
                            1 => { record.stmt.acct.svcr.fin_inst_id.bic =
                                DocumentMt940::parse_field_one(&capture[1].to_string());},
                            2 => {
                                DocumentMt940::parse_field_two(&capture[1].to_string(),
                                                               &mut record);
                            },
                            4 => { DocumentMt940::parse_field_foo(&capture[1].to_string(),
                                                                  &mut record); },
                            _ => {}
                        }
                    }
                    None => {}
                }
            }
        }
        Some(record)
    }

    fn from_mt940(buf_read: Stdin) -> Result<Self, ParserError> {
        let mut regex_pattern = String::new();
        if let Some(records) = DocumentMt940::find_record(&regex_pattern) {
            for record in records {
                DocumentMt940::parse_one_record(&regex_pattern[record.0..record.1]);
            }
        }
        Err(ParserError::BadInputFormatFile("No implement parse document".to_string()))
    }
}

impl DocumentCsv {
    fn from_csv(buf_read: Stdin) -> Result<Self, ParserError> {
        let reader = buf_read.lock();
        let mut csv_document: DocumentCsv = DocumentCsv::new();
        let mut csv_rdr = Reader::from_reader(reader);
        for row in csv_rdr.deserialize() {
            let row_data: RowCsv = row?;
            csv_document.rows.push(row_data);
        }
        Ok(csv_document)
    }
}
