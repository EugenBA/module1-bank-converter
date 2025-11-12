use std::env;
use bank_converter::converter::parser::{PipelineConverter};

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
    while args.len() > 0 
    {
        match args.remove(0).as_str(){
            "-i" => {
                //args_result.data_in.file_name = args.remove(0)
            }
            "-o" => {
                args_result.data_out.file_name = args.remove(0);
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
}