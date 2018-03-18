extern crate regex;

use std::env;
use code_file::source_code::SourceCode;
mod lexer;
use lexer::lexer::Lexer;
mod token;

fn main()
{
  let file_name = match env::args().nth(1)
  {
    Some(t) => t,
    _ => panic!("no file passed as arg"),
  };

  let mut lexer = Lexer::new(file_name)?;
  lexer.run();
}
