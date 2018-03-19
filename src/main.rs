extern crate regex;

use std::env;
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

  let mut lexer = match Lexer::new(file_name)
  {
    Ok(t) => t,
    Err(_) => panic!("no file found or cant read"),
  };
  lexer.run();
}
