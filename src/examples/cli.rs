use std::{env, io};
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::Path;

#[derive(PartialEq)]
pub enum FormatType {
    None,
    Csv,
    Xml,
    Mt940,
    Camt053,
}

pub struct PipelineConverter<T: Read, W: Write> {
    pub data_in: InputDataType<T>,
    pub data_out: OutDataType<W>
}

pub struct OutDataType<T:Write> {
    pub format_type: FormatType,
    pub(crate) buff_write: Option<T>,
}

pub struct InputDataType<T:Read> {
    pub format_type: FormatType,
    pub buff_read: Option<T>,
}

impl<T:Read, W:Write>  PipelineConverter<T, W> {
    pub fn get_format_type_from_string(format_str: &String) -> FormatType {
        match format_str.to_lowercase().as_str() {
            "csv" => FormatType::Csv,
            "xml" => FormatType::Xml,
            "mt940" => FormatType::Mt940,
            "camt053" => FormatType::Camt053,
            _ => FormatType::None
        }
    }
}


fn main() {
    // Получаем аргументы командной строки
    let mut args: Vec<String> = env::args().collect();

    // Если аргументов недостаточно, показываем справку
    if args.len() < 2 {
        eprintln!("Использование:");
        eprintln!("  -i <file name>");
        eprintln!("  -o <file name>");
        eprintln!("  --in_format CSV|XML|MT940|CAMT.053");
        eprintln!("  --out_format CSV|XML|MT940|CAMT.053");
        return;
    }
    let mut args_result = PipelineConverter::default();
    let mut in_file = String::new();
    let mut out_file = String::new();
    while args.len() > 0 
    {
        match args.remove(0).as_str(){
            "-i" => {
                in_file = args.remove(0);
            }
            "-o" => {
                out_file = args.remove(0);
            }
            "--in_format" => {
                let format = args.remove(0);
                args_result.data_in.format_type = PipelineConverter::get_format_type_from_string(&format);
            }
            "--out_format" => {
                let format = args.remove(0);
                args_result.data_out.format_type = PipelineConverter::get_format_type_from_string(&format);
            }
            _ => {
                eprintln!("Неизвестная команда");
                return;
            }
        }
    }
    if in_file.is_empty() || out_file.is_empty()  {
        eprintln!("Не указаны входной или выходной файл");
        return;
    }
    if args_result.data_in.format_type == FormatType::None ||
        args_result.data_out.format_type == FormatType::None {
        eprintln!("Не указаны форматы входного или выходного файла");
    }
    if args_result.data_in.format_type == args_result.data_out.format_type {
        eprintln!("Выбран один и тот же формат для входного и выходного файлов");
    }
    if !Path::new(&in_file).exists() {
        eprintln!("Файл {} не существует", in_file);
        return;
    }
    let file = File::open(in_file).unwrap();
    //args_result.data_in.buff_read = BufReader::new(file).;
   
}