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

    match file.read_to_string(&mut file_content)
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

  }

  fn next_token(&self, mut line_str: String, line: usize)
                -> (Vec<Token>, String)
  {
    let mut cut_pos: usize = 0;
    let mut regx_name = String::new();

    /*self.regex_list.iter().map(|rgx|  {
      rgx.captures(line_str.as_str()).iter();
    });*/

    for rgx in self.regex_list.iter()
    {
      println!("hiho {:?}", rgx);
      let caps = match rgx.captures(line_str.as_str())
      {
        Some(t) => t,
        None => continue,
      };

      if caps.len() == 0
      {
        continue;
      }

      println!("could come here");

      let firstMatch = match caps.get(0)
      {
        Some(t) => t,
        None => continue,
      };

      if firstMatch.start() == 0
      {
        println!("{:?}", firstMatch);
        cut_pos = firstMatch.end();
        regx_name = rgx.as_str().to_string();
        break;
      }

    }

    let tk_str: String = if regx_name == grammar::DREAD
    {
      let str_ret = line_str.drain(..line_str.len()).collect();
      str_ret
    }
    else
    {
      let str_ret = line_str.drain(..cut_pos).collect();
      str_ret
    };

    let tk_string = tk_str;
    let tks_types = Lexer::tk_type_from_str(&regx_name);
    let mut tk_vec:Vec<Token> = Vec::new();

      tks_types.iter().for_each(
         |tkt| {
         let tk_cp = *tkt;
         tk_vec.push(Token{line: line, col: cut_pos, str_tk: regx_name.clone(),
                           kind: tk_cp,
                           neg: Lexer::tk_is_str_neg(tk_string, &tkt) });
         }
      );

    return (tk_vec, line_str);
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

  fn tk_is_str_neg(tk_str: String, tk: &TokenTypes) -> bool
  {
    match *tk
    {
      TokenTypes::Oprp => return tk_str == "+",
      TokenTypes::Oprm => return tk_str == "*",
      TokenTypes::OprlrLgt => return tk_str == ">",
      TokenTypes::OprlrLgtEq => return tk_str == ">=",
      TokenTypes::Oprm => return tk_str == "==",
      _ => true,
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
