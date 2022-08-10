use clap::Parser;

use std::fs;
use std::io;
use std::io::Read;

mod language;

const TEMP_OUT: &str = r#"
global _start

section .data
    message: db "Hello World!", 0xA
       .len: equ $-message

section .text
_start:
    mov rax, 0x01
    mov rdi, 0x01
    mov rsi, message
    mov rdx, message.len
    syscall
    
    mov rax, 0x3C
    mov rdi, 0x00
    syscall  
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
            let tokens = language::lexer::tokenize(&source);
            for token in tokens {
                println!("{:?}", token);
            }

            let write = fs::write(&output_path, TEMP_OUT);
            match write {
                Err(err) => println!("[ERR | IO]: {}", err),
                _ => {}
            }
        }
        Err(err) => println!("[ERR | IO]: {}", err),
    }
}
