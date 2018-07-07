extern crate regex;

use token::tokens::Token;
use lexer::lexemes;
use token::tokens::TokenTypes;

use std::vec;
use std::fs::File;
use std::io::Read;
use std::string::String;

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

pub struct Lexer {
  regex_list: Vec<regex::Regex>,
  src: File,
  line_pos: usize,
  col_pos: usize
}

impl Lexer {

pub fn new(file_path: String) -> Result<Lexer, LexerError> {

  let mut rgx_list: Vec<regex::Regex> = Vec::new();
  rgx_list.push(regex::Regex::new("( )+").unwrap());
  rgx_list.push(regex::Regex::new(lexemes::FOR).unwrap());
  rgx_list.push(regex::Regex::new(lexemes::WHILE).unwrap());
  rgx_list.push(regex::Regex::new(lexemes::IF).unwrap());
  rgx_list.push(regex::Regex::new(lexemes::ELSE).unwrap());

  rgx_list.push(regex::Regex::new(lexemes::OPRL_OR).unwrap());
  rgx_list.push(regex::Regex::new(lexemes::OPRL_AND).unwrap());
  rgx_list.push(regex::Regex::new(lexemes::OPRL_NOT).unwrap());

  rgx_list.push(regex::Regex::new(lexemes::INT).unwrap());
  rgx_list.push(regex::Regex::new(lexemes::FLOAT).unwrap());
  rgx_list.push(regex::Regex::new(lexemes::CHAR).unwrap());
  rgx_list.push(regex::Regex::new(lexemes::STRING).unwrap());
  rgx_list.push(regex::Regex::new(lexemes::VEC).unwrap());
  rgx_list.push(regex::Regex::new(lexemes::VOID).unwrap());

  rgx_list.push(regex::Regex::new(lexemes::OP_BRACKET).unwrap());
  rgx_list.push(regex::Regex::new(lexemes::CL_BRACKET).unwrap());
  rgx_list.push(regex::Regex::new(lexemes::OP_CURLY).unwrap());
  rgx_list.push(regex::Regex::new(lexemes::CL_CURLY).unwrap());
  rgx_list.push(regex::Regex::new(lexemes::OP_PARENT).unwrap());
  rgx_list.push(regex::Regex::new(lexemes::CL_PARENT).unwrap());
  rgx_list.push(regex::Regex::new(lexemes::COMMA).unwrap());
  rgx_list.push(regex::Regex::new(lexemes::SEMICOLON).unwrap());

  rgx_list.push(regex::Regex::new(lexemes::OPR_PP).unwrap());

  rgx_list.push(regex::Regex::new(lexemes::OPRP).unwrap());
  rgx_list.push(regex::Regex::new(lexemes::OPRM).unwrap());

  rgx_list.push(regex::Regex::new(lexemes::OPRLR_LGT_EQ).unwrap());
  rgx_list.push(regex::Regex::new(lexemes::OPRLR_LGT).unwrap());
  rgx_list.push(regex::Regex::new(lexemes::OPRLR_EQ).unwrap());

  rgx_list.push(regex::Regex::new(lexemes::PRINT_FUN).unwrap());
  rgx_list.push(regex::Regex::new(lexemes::COMMENT).unwrap());
  rgx_list.push(regex::Regex::new(lexemes::ID).unwrap());

  rgx_list.push(regex::Regex::new(lexemes::ATTR_TO).unwrap());

  rgx_list.push(regex::Regex::new(lexemes::FLOAT_CONSTANT).unwrap());
  rgx_list.push(regex::Regex::new(lexemes::INT_CONSTANT).unwrap());
  rgx_list.push(regex::Regex::new(lexemes::CHAR_CONSTANT).unwrap());
  rgx_list.push(regex::Regex::new(lexemes::STRING_CONSTANT).unwrap());

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

  let mut ret = Lexer{regex_list: rgx_list, src: file, col_pos: 0,
                      line_pos: 0};
  return Ok(ret);
}

pub fn next_token(& mut self) -> Result<Token, LexerError>
{
  let mut single_char:Vec<u8> = Vec::with_capacity(4);
  single_char.push(0);
  println!("lol men {}", single_char.len());
  let mut token_string = String::new();
  let res_read = match self.src.read_exact(single_char.as_mut()) {
    Ok(t) => t,
    Err(e) => panic!(e),
  };
  println!("tk :{}", single_char[0] as char);

  Ok(Token{line:0, col:0, kind: TokenTypes::AttrTo, inverse: false,
           str_tk: String::new()})
}

  fn tk_type_from_str(s: &String) -> Result<TokenTypes, LexerError>
  {
    match s.as_str()
    {
      lexemes::FOR             => Ok(TokenTypes::ForKey),
      lexemes::WHILE           => Ok(TokenTypes::WhileKey),
      lexemes::IF              => Ok(TokenTypes::IfKey),
      lexemes::ELSE            => Ok(TokenTypes::ElseKey),
      lexemes::RETURN          => Ok(TokenTypes::RetKey),

      lexemes::OPRL_OR         => Ok(TokenTypes::OprlOr),
      lexemes::OPRL_AND        => Ok(TokenTypes::OprlAnd),
      lexemes::OPRL_NOT        => Ok(TokenTypes::OprlNot),

      lexemes::VOID            => Ok(TokenTypes::VoidKey),
      lexemes::INT             => Ok(TokenTypes::TypeInt),
      lexemes::CHAR            => Ok(TokenTypes::TypeChar),
      lexemes::FLOAT           => Ok(TokenTypes::TypeFloat),
      lexemes::STRING          => Ok(TokenTypes::TypeStr),
      lexemes::VEC             => Ok(TokenTypes::TypeVec),

      lexemes::OP_BRACKET      => Ok(TokenTypes::OpBracks),
      lexemes::CL_BRACKET      => Ok(TokenTypes::ClBracks),
      lexemes::OP_CURLY        => Ok(TokenTypes::OpCBracks),
      lexemes::CL_CURLY        => Ok(TokenTypes::ClCBracks),
      lexemes::OP_PARENT       => Ok(TokenTypes::OpPar),
      lexemes::CL_PARENT       => Ok(TokenTypes::ClPar),
      lexemes::COMMA           => Ok(TokenTypes::Comma),
      lexemes::SEMICOLON       => Ok(TokenTypes::Semicolon),

      lexemes::ATTR_TO         => Ok(TokenTypes::AttrTo),

      lexemes::OPR_PP          => Ok(TokenTypes::OprPP),

      lexemes::PRINT_FUN       => Ok(TokenTypes::PrintFun),

      lexemes::OPRP            => Ok(TokenTypes::Oprp),
      lexemes::OPRM            => Ok(TokenTypes::Oprm),
      lexemes::OPRLR_LGT       => Ok(TokenTypes::OprlrLgt),
      lexemes::OPRLR_LGT_EQ    => Ok(TokenTypes::OprlrLgtEq),
      lexemes::OPRLR_EQ        => Ok(TokenTypes::OprlrEq),
      lexemes::COMMENT         => Ok(TokenTypes::Comment),
      lexemes::ID              => Ok(TokenTypes::ID),


      lexemes::FLOAT_CONSTANT  => Ok(TokenTypes::FloatConst),
      lexemes::INT_CONSTANT    => Ok(TokenTypes::IntConst),
      lexemes::CHAR_CONSTANT   => Ok(TokenTypes::CharConst),
      lexemes::STRING_CONSTANT => Ok(TokenTypes::StrConst),

      _ => Err(LexerError
        {
          cause: LexerErrorType::UnknowCharSeq,
          message: String::from("unknow token at line: {}, colunm: {}"),
        })
    }
  }

  fn is_str_an_inverse_tk(tk_str: &String, tk: &TokenTypes) -> bool
  {
    match *tk
    {
      TokenTypes::Oprp => return tk_str == "-",
      TokenTypes::Oprm => return tk_str == "/",
      TokenTypes::OprlrLgt => return tk_str == "<",
      TokenTypes::OprlrLgtEq => return tk_str == "<=",
      TokenTypes::OprlrEq => return tk_str == "!=",
      _ => false,
    }
  }

  fn create_line_list(file_content: String) -> Vec<String>
  {
    let src_in_lines: Vec<_> =
          file_content.lines()
                      .map(|x| String::from(x))
                      .collect();

    return src_in_lines;
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
