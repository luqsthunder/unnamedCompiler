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

  let mut lexer = match Lexer::new(file_name)
  {
    Ok(t) => t,
    Err(_) => panic!("no file found or cant read"),
  };

  let mut curr_tk = Token{kind:TokenTypes::ID, line: 1, col: 1, inverse: false,
                          str_tk: String::new() };
  let mut curr_tk_kind = curr_tk.kind.clone();
  while curr_tk_kind != TokenTypes::Eof
  {
    curr_tk = match lexer.next_token()
    {
      Ok(t) => t,
      Err(e) => panic!(e),
    };

    Lexer::print_token_as_alcino_likes(&curr_tk);
    curr_tk_kind = curr_tk.kind;
  }
}
