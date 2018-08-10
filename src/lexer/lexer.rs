extern crate regex;

use token::tokens::Token;
use lexer::lexemes;
use token::tokens::TokenTypes;

use std::vec;
use std::fs::File;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Read;
use std::string::String;
use std::collections::HashMap;

#[derive(Debug)]
pub enum LexerErrorType {
  OpenFile,
  ReadFile,
  UnknowCharSeq,
}

#[derive(Debug)]
pub struct LexerError {
  pub cause: LexerErrorType,
  pub message: String,
}

#[derive(PartialEq)]
enum LexGroup {
  Empty,
  AlphaNumeric,
  Numeric,
  Symbol,
}

pub struct Lexer {
  regex_list: Vec<regex::Regex>,
  lexemes_tokens: HashMap<String, TokenTypes>,
  src: File,
  line_pos: usize,
  col_pos: usize,
}

impl Lexer {

pub fn new(file_path: String) -> Result<Lexer, LexerError> {

  let mut rgx_list: Vec<regex::Regex> = Vec::new();
  rgx_list.push(regex::Regex::new("( )+").unwrap());
  for l in lexemes::ALL_VEC.iter() {
    rgx_list.push(regex::Regex::new(l).unwrap());
  }

  let mut file = match File::open(&file_path) {
    Ok(t) => t,
    Err(err) => {
      println!("{}", err);
      let error = LexerError {
        message: String::from("cant open file: "),// + file_path.as_str(),
        cause: LexerErrorType::OpenFile,
      };
      return Err(error);
    },
  };
  let mut lex_tk:HashMap<String, TokenTypes> = HashMap::new();
  lex_tk.insert(String::from("int"), TokenTypes::TypeInt);

  let mut ret = Lexer{regex_list: rgx_list, src: file, col_pos: 0,
                       line_pos: 0, lexemes_tokens: lex_tk};
  return Ok(ret);
}

pub fn next_token(& mut self) -> Result<Token, LexerError> {
  let mut token_string:String = String::new();
  token_string.reserve(10);
  let mut s_char:Vec<u8> = Vec::with_capacity(1);
  s_char.push(0);

  let mut lex_group = LexGroup::Empty;

  loop {
    let read_size = match self.src.read_exact(s_char.as_mut()) {
      Ok(t) => t,
      Err(e) => panic!(e),
    };
    // alpha-numeric tokens e.g id, int, numbers
    if (s_char[0] >= ('0' as u8) && s_char[0] <= ('9' as u8)) ||
       (s_char[0] >= ('a' as u8) && s_char[0] <= ('z' as u8)) ||
       (s_char[0] >= ('A' as u8) && s_char[0] <= ('Z' as u8)) ||
       (s_char[0] == ('_' as u8)) {

      if lex_group != LexGroup::AlphaNumeric && lex_group != LexGroup::Empty {
        self.src.seek(SeekFrom::Current(-1));
        break;
      }

      token_string.push(s_char[0] as char);
      lex_group = LexGroup::AlphaNumeric;
      continue;
    }
    else if s_char[0] != (' ' as u8) && s_char[0] != ('\n' as u8) {

      if lex_group != LexGroup::Symbol && lex_group != LexGroup::Empty {
        self.src.seek(SeekFrom::Current(-1));
        break;
      }

      token_string.push(s_char[0] as char);
      lex_group = LexGroup::Symbol;
      continue
    }

    if !token_string.is_empty() {
      break;
    }
  }

  Ok(Token{line:0, col:0, kind: TokenTypes::AttrTo, str_tk: String::new()})
}

pub fn print_token_as_alcino_likes(tk: &Token)
{
  let kt = tk.clone();
  let kind_value = kt.kind.clone() as u8;
  let kind_str = kt.kind.to_string();
  println!("[{:#03}, {:#03}] ({:#04}, {:10}) {{{}}}", kt.line, kt.col,
           kind_value, kind_str, kt.str_tk);
}

}
