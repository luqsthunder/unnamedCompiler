extern crate regex;

use std::env;
mod lexer;
use lexer::lexer::Lexer;
use token::tokens::TokenTypes;
use token::tokens::Token;
mod token;

fn main()
{
  let file_name = match env::args().nth(1)
  {
    Some(t) => t,
    _ => panic!("no file passed as arg"),
  };

  println!("{}", file_name);
  let mut lexer = match Lexer::new(String::from("./fibonacci.un"))
  {
    Ok(t) => t,
    Err(_) => panic!("no file found or cant read"),
  };

  lexer.next_token();
  lexer.next_token();
  lexer.next_token();
  lexer.next_token();
}
