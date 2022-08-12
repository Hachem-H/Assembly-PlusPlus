use clap::Parser;

use std::fs;
use std::io;
use std::io::Read;

mod language;
mod tables;

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

            let mut tokens = language::lexer::tokenize(&source);
            let output = language::interpret(&mut runtime, &mut tokens);

            match output {
                Ok(output) => {
                    println!("{}", output);

                    let write = fs::write(&output_path, output);
                    match write {
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
