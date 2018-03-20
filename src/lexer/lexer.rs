extern crate regex;

use token::tokens::Token;
use token::grammar;
use token::tokens::TokenTypes;
use std::fs::File;
use std::io::Read;
use std::string::String;

use std::collections::LinkedList;

pub enum FileErrorType
{
  OPEN,
  READ,
}

pub struct FileLexerError
{
  pub cause: FileErrorType,
  pub message: String,
}

pub struct Lexer
{
  tokens_list: LinkedList<Token>,
  regex_list: Vec<regex::Regex>,
  src: Vec<String>
}

impl Lexer
{
  pub fn new(file_path: String) -> Result<Lexer, FileLexerError>
  {

    let mut rgx_list: Vec<regex::Regex> = Vec::new();
    rgx_list.push(regex::Regex::new("( )+").unwrap());
    rgx_list.push(regex::Regex::new(grammar::FOR).unwrap());
    rgx_list.push(regex::Regex::new(grammar::WHILE).unwrap());
    rgx_list.push(regex::Regex::new(grammar::IF).unwrap());
    rgx_list.push(regex::Regex::new(grammar::ELSE).unwrap());

    rgx_list.push(regex::Regex::new(grammar::INT).unwrap());
    rgx_list.push(regex::Regex::new(grammar::FLOAT).unwrap());
    rgx_list.push(regex::Regex::new(grammar::CHAR).unwrap());
    rgx_list.push(regex::Regex::new(grammar::STRING).unwrap());
    rgx_list.push(regex::Regex::new(grammar::VEC).unwrap());

    rgx_list.push(regex::Regex::new(grammar::OP_BRACKET).unwrap());
    rgx_list.push(regex::Regex::new(grammar::CL_BRACKET).unwrap());
    rgx_list.push(regex::Regex::new(grammar::OP_CURLY).unwrap());
    rgx_list.push(regex::Regex::new(grammar::CL_CURLY).unwrap());
    rgx_list.push(regex::Regex::new(grammar::OP_PARENT).unwrap());
    rgx_list.push(regex::Regex::new(grammar::CL_PARENT).unwrap());
    rgx_list.push(regex::Regex::new(grammar::COMMA).unwrap());
  
    rgx_list.push(regex::Regex::new(grammar::OPRP).unwrap());
    rgx_list.push(regex::Regex::new(grammar::OPRM).unwrap());

    rgx_list.push(regex::Regex::new(grammar::OPRLR_LGT).unwrap());
    rgx_list.push(regex::Regex::new(grammar::OPRLR_LGT_EQ).unwrap());
    rgx_list.push(regex::Regex::new(grammar::OPRLR_EQ).unwrap());

    rgx_list.push(regex::Regex::new(grammar::DREAD).unwrap());
    rgx_list.push(regex::Regex::new(grammar::ID).unwrap());

    rgx_list.push(regex::Regex::new(grammar::FLOAT_CONSTANT).unwrap());
    rgx_list.push(regex::Regex::new(grammar::INT_CONSTANT).unwrap());
    rgx_list.push(regex::Regex::new(grammar::CHAR_CONSTANT).unwrap());
    rgx_list.push(regex::Regex::new(grammar::STRING_CONSTANT).unwrap());

    let ret = Lexer{tokens_list: LinkedList::new(), regex_list: rgx_list,
                    src: Vec::new()};

    let mut file = match File::open(&file_path)
    {
      Ok(t) => t,
      Err(_) =>
      {
        let error = FileLexerError {
          message: String::from("cant open file: "),// + file_path.as_str(),
          cause: FileErrorType::OPEN,
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
        let error =  FileLexerError {
          message: String::from("cant read file: "),
          cause: FileErrorType::READ,
        };
        return Err(error);
      },
    };

    return Ok(ret);
  }

  pub fn tokens(self) -> LinkedList<Token>
  {
    return self.tokens_list;
  }

  pub fn run(&mut self)
  {
    let line = String::from("int functionlol(int a, int b)");
    let (tk, nstr) = self.next_token(line, 0);
    println!("{:?} {}", tk, nstr);

    let (tk2, nstr2) = self.next_token(nstr, 0);
    println!("{:?} {}", tk2, nstr2);

    let (tk3, nstr3) = self.next_token(nstr2, 0);
    println!("{:?} {}", tk3, nstr3);
  }

  fn next_token(&self,line_str: String, line: usize)
                -> (Vec<Token>, String)
  {
    let mut cut_pos: usize = 0;
    let mut regx_name = String::new();
    let mut tk_str = String::new();
    let mut line_str_cp = line_str.clone();

    let mut white = false;
    let mut end_rgx = false;

    let mut curr_line = line_str.clone();

    while ((! white) && (! end_rgx))
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
            break;
          }
          regx_name = rgx.as_str().to_string();

          white = true;
          end_rgx = true;
          break;
        }
      }
      curr_line = line_str_cp.clone();
    }

    if tk_str == "#"
    {
      line_str_cp.drain(..tk_str.len());
    }
    else
    {
      line_str_cp.drain(..cut_pos);
    }

    let tks:Vec<Token> =
      Lexer::tk_type_from_str(&tk_str).iter()
                                     .map(|t| Token{
                                       line: line,
                                       col: cut_pos,
                                       kind: t.clone(),
                                       neg: Lexer::tk_is_str_neg(&tk_str, t),
                                       str_tk: regx_name.clone(),
                                     })
                                     .collect();
    println!("return {}", line_str_cp);
    (tks, line_str_cp)
  }

  fn tk_type_from_str(s: &String) -> Vec<TokenTypes>
  {
    match s.as_str()
    {
      grammar::FOR => vec![TokenTypes::ForKey],
      grammar::WHILE => vec![TokenTypes::WhileKey],
      grammar::IF => vec![TokenTypes::IfKey],
      grammar::ELSE => vec![TokenTypes::ElseKey],
      grammar::INT => vec![TokenTypes::TypeInt],
      grammar::CHAR => vec![TokenTypes::TypeChar],
      grammar::FLOAT => vec![TokenTypes::TypeFloat],
      grammar::STRING => vec![TokenTypes::TypeChar],
      grammar::VEC => vec![TokenTypes::TypeVec],
      grammar::OP_BRACKET => vec![TokenTypes::OpBrackets],
      grammar::CL_BRACKET => vec![TokenTypes::ClBrackets],
      grammar::OP_CURLY => vec![TokenTypes::OpCurlyBrackets],
      grammar::CL_CURLY => vec![TokenTypes::ClCurlyBrackets],
      grammar::OP_PARENT => vec![TokenTypes::OpParenthesys],
      grammar::CL_PARENT => vec![TokenTypes::ClParenthesys],
      grammar::COMMA => vec![TokenTypes::Comma],
      grammar::OPRP => vec![TokenTypes::Oprp],
      grammar::OPRM => vec![TokenTypes::Oprm],
      grammar::OPRLR_LGT => vec![TokenTypes::OprlrLgt],
      grammar::OPRLR_LGT_EQ => vec![TokenTypes::OprlrLgtEq],
      grammar::OPRLR_EQ => vec![TokenTypes::OprlrEq],
      grammar::DREAD => vec![TokenTypes::Dread],
      grammar::ID => vec![TokenTypes::ID],
      grammar::INT_CONSTANT => vec![TokenTypes::IntConst],
      grammar::FLOAT_CONSTANT => vec![TokenTypes::FloatConst],
      grammar::CHAR_CONSTANT => vec![TokenTypes::CharConst],
      grammar::STRING_CONSTANT => vec![TokenTypes::StringConst],
      "string" => vec![TokenTypes::TypeChar, TokenTypes::TypeVec],
      _ => vec![TokenTypes::Err],
    }
  }

  fn tk_is_str_neg(tk_str: &String, tk: &TokenTypes) -> bool
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

}
