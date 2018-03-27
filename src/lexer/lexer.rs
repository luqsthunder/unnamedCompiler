extern crate regex;

use token::tokens::Token;
use token::grammar;
use token::tokens::TokenTypes;
use std::fs::File;
use std::io::Read;
use std::string::String;

#[derive(Debug)]
pub enum LexerErrorType
{
  OpenFile,
  ReadFile,
  UnknowCharSeq,
}

#[derive(Debug)]
pub struct LexerError
{
  pub cause: LexerErrorType,
  pub message: String,
}

pub struct Lexer
{
  regex_list: Vec<regex::Regex>,
  src: Vec<String>,
  src_line: String,
  line_pos: usize,
  col_pos: usize
}

impl Lexer
{
  pub fn new(file_path: String) -> Result<Lexer, LexerError>
  {

    let mut rgx_list: Vec<regex::Regex> = Vec::new();
    rgx_list.push(regex::Regex::new("( )+").unwrap());
    rgx_list.push(regex::Regex::new(grammar::FOR).unwrap());
    rgx_list.push(regex::Regex::new(grammar::WHILE).unwrap());
    rgx_list.push(regex::Regex::new(grammar::IF).unwrap());
    rgx_list.push(regex::Regex::new(grammar::ELSE).unwrap());

    rgx_list.push(regex::Regex::new(grammar::OPRL_OR).unwrap());
    rgx_list.push(regex::Regex::new(grammar::OPRL_AND).unwrap());
    rgx_list.push(regex::Regex::new(grammar::OPRL_NOT).unwrap());

    rgx_list.push(regex::Regex::new(grammar::INT).unwrap());
    rgx_list.push(regex::Regex::new(grammar::FLOAT).unwrap());
    rgx_list.push(regex::Regex::new(grammar::CHAR).unwrap());
    rgx_list.push(regex::Regex::new(grammar::STRING).unwrap());
    rgx_list.push(regex::Regex::new(grammar::VEC).unwrap());
    rgx_list.push(regex::Regex::new(grammar::VOID).unwrap());

    rgx_list.push(regex::Regex::new(grammar::OP_BRACKET).unwrap());
    rgx_list.push(regex::Regex::new(grammar::CL_BRACKET).unwrap());
    rgx_list.push(regex::Regex::new(grammar::OP_CURLY).unwrap());
    rgx_list.push(regex::Regex::new(grammar::CL_CURLY).unwrap());
    rgx_list.push(regex::Regex::new(grammar::OP_PARENT).unwrap());
    rgx_list.push(regex::Regex::new(grammar::CL_PARENT).unwrap());
    rgx_list.push(regex::Regex::new(grammar::COMMA).unwrap());
    rgx_list.push(regex::Regex::new(grammar::SEMICOLON).unwrap());

    rgx_list.push(regex::Regex::new(grammar::OPR_PP).unwrap());

    rgx_list.push(regex::Regex::new(grammar::OPRP).unwrap());
    rgx_list.push(regex::Regex::new(grammar::OPRM).unwrap());

    rgx_list.push(regex::Regex::new(grammar::OPRLR_LGT_EQ).unwrap());
    rgx_list.push(regex::Regex::new(grammar::OPRLR_LGT).unwrap());
    rgx_list.push(regex::Regex::new(grammar::OPRLR_EQ).unwrap());

    rgx_list.push(regex::Regex::new(grammar::PRINT_FUN).unwrap());
    rgx_list.push(regex::Regex::new(grammar::COMMENT).unwrap());
    rgx_list.push(regex::Regex::new(grammar::ID).unwrap());

    rgx_list.push(regex::Regex::new(grammar::ATTR_TO).unwrap());

    rgx_list.push(regex::Regex::new(grammar::FLOAT_CONSTANT).unwrap());
    rgx_list.push(regex::Regex::new(grammar::INT_CONSTANT).unwrap());
    rgx_list.push(regex::Regex::new(grammar::CHAR_CONSTANT).unwrap());
    rgx_list.push(regex::Regex::new(grammar::STRING_CONSTANT).unwrap());

    let mut ret = Lexer{regex_list: rgx_list, src: Vec::new(), col_pos: 0,
                        src_line: String::new(), line_pos: 0};

    let mut file = match File::open(&file_path)
    {
      Ok(t) => t,
      Err(_) =>
      {
        let error = LexerError {
          message: String::from("cant open file: "),// + file_path.as_str(),
          cause: LexerErrorType::OpenFile,
        };
        return Err(error);
      },
    };

    let mut file_content = String::new();

    match file.read_to_string(& mut file_content)
    {
      Ok(t) => t,
      Err(_) =>
      {
        let error =  LexerError {
          message: String::from("cant read file: "),
          cause: LexerErrorType::ReadFile,
        };
        return Err(error);
      },
    };

    ret.src = Lexer::create_line_list(file_content);
    return Ok(ret);
  }

  pub fn next_token(& mut self) -> Result<Token, LexerError>
  {
    let mut cut_pos: usize = 0;
    let mut rgx_nm = String::new();
    let mut tk_str = String::new();

    let mut found = false;
    let mut space = false;


    if self.src_line.is_empty()
    {
      if self.line_pos < self.src.len()
      {
        while self.src[self.line_pos].split_whitespace().next().is_none()
        {
          self.line_pos += 1;
          if self.line_pos == self.src.len() - 1
          {
            break;
          }
        }
      }
      else if self.line_pos == self.src.len()
      {
        return Ok(Token{line: self.line_pos + 1,
          col: cut_pos + self.col_pos, kind: TokenTypes::Eof,
          inverse: false, str_tk: String::new(),
        });
      }

      self.src_line = self.src[self.line_pos].clone();
    }

    let mut line_str_cp = self.src_line.clone();
    let mut curr_line = self.src_line.clone();

    while ! found
    {
      for rgx in self.regex_list.iter()
      {
        let caps = match rgx.captures(curr_line.as_str())
        {
          Some(t) => t,
          None => continue,
        };

        if caps.len() == 0
        {
          continue;
        }

        let first_match = match caps.get(0)
        {
          Some(t) => t,
          None => continue,
        };

        if first_match.start() == 0
        {
          tk_str.push_str(first_match.as_str());
          cut_pos = first_match.end();
          if tk_str.chars().nth(0).unwrap() == ' '
          {
            tk_str.clear();
            line_str_cp.drain(..cut_pos);
            space = true;
            break;
          }
          rgx_nm = rgx.as_str().to_string();

          found = true;
          break;
        }
      }
      if (! found) && (! space)
      {
        break;
      }
      space = false;
      curr_line = line_str_cp.clone();
    }

    if tk_str == "#"
    {
      tk_str = line_str_cp.drain(..curr_line.len()).collect();
    }
    else
    {
      tk_str = line_str_cp.drain(..cut_pos).collect();
    }

    if ! line_str_cp.split_whitespace().next().is_some()
    {
      line_str_cp = String::from("");
      if self.line_pos == self.src.len() -1
      {
        return Ok(Token{col: self.col_pos, line: self.line_pos,
                        kind: TokenTypes::Eof, inverse: false,
                        str_tk: String::new()});
      }
    }

    let tk_kind = match Lexer::tk_type_from_str(&rgx_nm)
    {
      Ok(t) => t,
      Err(e) => return Err(e),
    };

    let tk:Token = Token{line: self.line_pos + 1,
      col: cut_pos + self.col_pos, kind: tk_kind.clone(),
      inverse: Lexer::is_str_an_inverse_tk(&tk_str, &tk_kind),
      str_tk: tk_str.clone(),
    };

    self.col_pos += cut_pos;
    if line_str_cp.is_empty()
    {
      self.line_pos += 1;
      self.col_pos = 0;
    }

    self.src_line = line_str_cp;
    Ok(tk)
  }

  fn tk_type_from_str(s: &String) -> Result<TokenTypes, LexerError>
  {
    match s.as_str()
    {
      grammar::FOR             => Ok(TokenTypes::ForKey),
      grammar::WHILE           => Ok(TokenTypes::WhileKey),
      grammar::IF              => Ok(TokenTypes::IfKey),
      grammar::ELSE            => Ok(TokenTypes::ElseKey),
      grammar::RETURN          => Ok(TokenTypes::RetKey),

      grammar::OPRL_OR         => Ok(TokenTypes::OprlOr),
      grammar::OPRL_AND        => Ok(TokenTypes::OprlAnd),
      grammar::OPRL_NOT        => Ok(TokenTypes::OprlNot),

      grammar::VOID            => Ok(TokenTypes::VoidKey),
      grammar::INT             => Ok(TokenTypes::TypeInt),
      grammar::CHAR            => Ok(TokenTypes::TypeChar),
      grammar::FLOAT           => Ok(TokenTypes::TypeFloat),
      grammar::STRING          => Ok(TokenTypes::TypeStr),
      grammar::VEC             => Ok(TokenTypes::TypeVec),

      grammar::OP_BRACKET      => Ok(TokenTypes::OpBracks),
      grammar::CL_BRACKET      => Ok(TokenTypes::ClBracks),
      grammar::OP_CURLY        => Ok(TokenTypes::OpCBracks),
      grammar::CL_CURLY        => Ok(TokenTypes::ClCBracks),
      grammar::OP_PARENT       => Ok(TokenTypes::OpPar),
      grammar::CL_PARENT       => Ok(TokenTypes::ClPar),
      grammar::COMMA           => Ok(TokenTypes::Comma),
      grammar::SEMICOLON       => Ok(TokenTypes::Semicolon),

      grammar::ATTR_TO         => Ok(TokenTypes::AttrTo),

      grammar::OPR_PP          => Ok(TokenTypes::OprPP),

      grammar::PRINT_FUN       => Ok(TokenTypes::PrintFun),

      grammar::OPRP            => Ok(TokenTypes::Oprp),
      grammar::OPRM            => Ok(TokenTypes::Oprm),
      grammar::OPRLR_LGT       => Ok(TokenTypes::OprlrLgt),
      grammar::OPRLR_LGT_EQ    => Ok(TokenTypes::OprlrLgtEq),
      grammar::OPRLR_EQ        => Ok(TokenTypes::OprlrEq),
      grammar::COMMENT         => Ok(TokenTypes::Comment),
      grammar::ID              => Ok(TokenTypes::ID),


      grammar::FLOAT_CONSTANT  => Ok(TokenTypes::FloatConst),
      grammar::INT_CONSTANT    => Ok(TokenTypes::IntConst),
      grammar::CHAR_CONSTANT   => Ok(TokenTypes::CharConst),
      grammar::STRING_CONSTANT => Ok(TokenTypes::StrConst),

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
