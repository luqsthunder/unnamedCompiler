#![allow(dead_code)]

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

    rgx_list.push(regex::Regex::new(grammar::DREAD).unwrap());
    rgx_list.push(regex::Regex::new(grammar::ID).unwrap());

    rgx_list.push(regex::Regex::new(grammar::ATTR_TO).unwrap());

    rgx_list.push(regex::Regex::new(grammar::NUMERIC_CONSTANT).unwrap());
    rgx_list.push(regex::Regex::new(grammar::CHAR_CONSTANT).unwrap());
    rgx_list.push(regex::Regex::new(grammar::STRING_CONSTANT).unwrap());

    let mut ret = Lexer{tokens_list: LinkedList::new(), regex_list: rgx_list,
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

    ret.src = Lexer::create_line_list(file_content);
    return Ok(ret);
  }

  pub fn tokens(self) -> LinkedList<Token>
  {
    return self.tokens_list;
  }

  pub fn run(&mut self)
  {
    for (idx, line) in self.src.iter().enumerate()
    {
      println!("line: {} {}", idx, line);
      let mut curr_line = line.clone();
      while ! curr_line.is_empty()
      {
        let (tks, new_str) = self.next_token(&curr_line, idx + 1,
                                             line.len() - curr_line.len());
        curr_line.clear();
        curr_line.insert_str(0, new_str.as_str());
        println!("{:?}", tks);
        for t in tks.into_iter()
        {
          if t.kind == TokenTypes::Err
          {
            panic!("wrong expression at \n {} \n line:{} col:{}", line, idx + 1,
                   line.len() - curr_line.len());
          }
          else
          {
            self.tokens_list.push_back(t);
          }
        }
      }
    }

    for tk in self.tokens_list.iter()
    {
      println!("{:?}", tk);
    }
  }

  fn next_token(&self,line_str: &String, line: usize, line_pos: usize)
                -> (Vec<Token>, String)
  {
    let mut cut_pos: usize = 0;
    let mut rgx_nm = String::new();
    let mut tk_str = String::new();
    let mut line_str_cp = line_str.clone();

    let mut found = false;
    let mut space = false;

    let mut curr_line = line_str.clone();

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
    }

    let tks:Vec<Token> =
      Lexer::tk_type_from_str(&rgx_nm).iter()
                                      .map(|t| Token{
                                        line: line,
                                        col: cut_pos + line_pos,
                                        kind: t.clone(),
                                        neg: Lexer::tk_is_str_neg(&tk_str, t),
                                        str_tk: tk_str.clone(),
                                      })
                                      .collect();
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
      grammar::RETURN => vec![TokenTypes::RetKey],

      grammar::OPRL_OR => vec![TokenTypes::OprlOr],
      grammar::OPRL_AND => vec![TokenTypes::OprlAnd],
      grammar::OPRL_NOT => vec![TokenTypes::OprlNot],

      grammar::VOID => vec![TokenTypes::VoidKey],
      grammar::INT => vec![TokenTypes::TypeInt],
      grammar::CHAR => vec![TokenTypes::TypeChar],
      grammar::FLOAT => vec![TokenTypes::TypeFloat],
      grammar::STRING => vec![TokenTypes::TypeChar, TokenTypes::TypeVec],
      grammar::VEC => vec![TokenTypes::TypeVec],

      grammar::OP_BRACKET => vec![TokenTypes::OpBrackets],
      grammar::CL_BRACKET => vec![TokenTypes::ClBrackets],
      grammar::OP_CURLY => vec![TokenTypes::OpCurlyBrackets],
      grammar::CL_CURLY => vec![TokenTypes::ClCurlyBrackets],
      grammar::OP_PARENT => vec![TokenTypes::OpParenthesys],
      grammar::CL_PARENT => vec![TokenTypes::ClParenthesys],
      grammar::COMMA => vec![TokenTypes::Comma],
      grammar::SEMICOLON => vec![TokenTypes::Semicolon],

      grammar::ATTR_TO => vec![TokenTypes::AttrTo],

      grammar::OPR_PP => vec![TokenTypes::OprPP],

      grammar::OPRP => vec![TokenTypes::Oprp],
      grammar::OPRM => vec![TokenTypes::Oprm],
      grammar::OPRLR_LGT => vec![TokenTypes::OprlrLgt],
      grammar::OPRLR_LGT_EQ => vec![TokenTypes::OprlrLgtEq],
      grammar::OPRLR_EQ => vec![TokenTypes::OprlrEq],
      grammar::DREAD => vec![TokenTypes::Dread],
      grammar::ID => vec![TokenTypes::ID],
      grammar::NUMERIC_CONSTANT => vec![TokenTypes::NumericConst],
      grammar::CHAR_CONSTANT => vec![TokenTypes::CharConst],
      grammar::STRING_CONSTANT => vec![TokenTypes::StringConst],
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
