use clap::Parser;

use std::fs;
use std::io;
use std::io::Read;

mod language;
mod tables;

const TEXT_HEADER: &str = "section .text\n";
const DATA_HEADER: &str = "section .data\n";
const BSS_HEADER: &str = "section .bss\n";

const ENTRY_POINT: &str = r#"
global _start
_start:
    call main

"#;

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut file = fs::File::open(path)?;
    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;
    Ok(buffer)
}

#[derive(Parser, Debug)]
#[clap(
    author = "Hachem H.",
    about = "Assembly++",
    long_about = "A higher level assembly scripting language"
)]
struct Arguments {
    #[clap(value_parser, short = 's', long = "source")]
    source_file: String,

    #[clap(
        value_parser,
        short = 'o',
        long = "output",
        default_value = "output.asm"
    )]
    output_file: String,
}

fn main() {
    let arguments = Arguments::parse();
    let file_path = arguments.source_file;
    let output_path = arguments.output_file;

    let file = read_file(&file_path);
    match file {
        Ok(source) => {
            let mut runtime = language::Runtime::new();
            let mut file_output = String::new();

            let mut tokens = language::lexer::tokenize(&source);
            let text_section = language::interpret(&mut runtime, &mut tokens);
            let data_section = language::generate_data_section(&runtime);
            let bss_section = language::generate_bss_section(&runtime);

            match text_section {
                Ok(text_section) => {
                    file_output.push_str(TEXT_HEADER);
                    file_output.push_str(&*text_section);
                    file_output.push_str(ENTRY_POINT);
                    file_output.push_str(DATA_HEADER);
                    file_output.push_str(&*data_section);
                    file_output.push_str(BSS_HEADER);
                    file_output.push_str(&*bss_section);
                    println!("{}", file_output);

                    match fs::write(&output_path, file_output) {
                        Err(err) => println!("[ERR | IO]: {}", err),
                        _ => {}
                    }
                }

                Err(error) => {
                    println!("[ERR]: {}", error);
                }
            }
        }
        Err(err) => println!("[ERR | IO]: {}", err),
    }
}
