#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::language::Token;
use crate::tables::*;

type Output = Result<String, String>;

struct Parser;
#[rustfmt::skip]
impl Parser {
    // TODO(Hachem): Implement.
    // NOTE(Hachem): The parser takes the current read index and the tokens as input.
    //               It starts by parsing out a specific statement whilst generating
    //               a new vector of tokens as form of expression. In doing so, it
    //               advances the index by the length of said vector. Then does the 
    //               proper parsing returing an output string which will be processed
    //               by the transpiler, (and by processed, I mean just pushed to the
    //               output buffer).
    //               This operation could fail (parsing errors) so we return a result
    //                  -> Result<Error, OutputString>

    fn parse_expression(index: &i32, tokens: Vec<Token>)  -> Output { Ok(String::new()) }
    fn parse_assigment(index: &i32, tokens: Vec<Token>)   -> Output { Ok(String::new()) }
    fn parse_conditional(index: &i32, tokens: Vec<Token>) -> Output { Ok(String::new()) }
    fn parse_loop(index: &i32, tokens: Vec<Token>)        -> Output { Ok(String::new()) }
}
