//! keywords:
//!    types: int, float, char, vec types-> int::, string(char::), float::
//!    statement keywords: if, else
//!

use std::fmt;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum TokenTypes
{
  ID,                // ID of a var, const value
  TypeInt,           // type integer
  TypeFloat,         // type float
  TypeChar,          // type character
  TypeVec,           // type vector
  VoidKey,           // void keyword
  ErrorMatch,               // only for pattern matching
  RetKey,            // return keyword
  ForKey,            // for keyword
  WhileKey,          // while keyword
  IfKey,             // if keyword
  ElseKey,           // keyword else
  PrintFun,          // print func
  Comment,             // # comment
  Oprm,              // * or /
  Oprp,              // + or -
  OprlOr,            // or
  OprlAnd,           // and
  OprlNot,           // not
  OprPP,             // ++
  OprlrEq,           // operator equals or not equals
  OprlrLgt,          // operator larger greater or less than
  OprlrLgtEq,        // operator larger greater or less than
  NumericConst,      // 1.29
  AttrTo,            // =
  CharConst,         // 'a'
  StringConst,       // "lol"
  OpBrackets,        // [
  ClBrackets,        // ]
  OpParenthesys,     // (
  ClParenthesys,     // )
  OpCurlyBrackets,   // {
  ClCurlyBrackets,   // }
  Comma,             // ,
  Semicolon,         // ;
}


impl fmt::Display for TokenTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self
		{			
		  TokenTypes::ID  =>  write!(f, "ID"),                // ID of a var, const value
		  //TokenTypes::TypeInt => "TypeInt",           // type integer
		  //TokenTypes::TypeFloat => "TypeFloat",         // type float
		  //TokenTypes::TypeChar => "TypeChar",          // type character
		  //TypeVec => "TypeVec",           // type vector
		  /*VoidKey => "VoidKey",           // void keyword
		  ErrorMatch => "Err",               // only for pattern matching
		  RetKey => "RetKey",            // return keyword
		  ForKey => "ForKey",            // for keyword
		  WhileKey => "WhileKey",          // while keyword
		  IfKey => "IfKey",             // if keyword
		  ElseKey => "ElseKey",           // keyword else
		  PrintFun => "PrintFun",          // print func
		  Comment => "Comment",             // # comment
		  Oprm => "Oprm",              // * or /
		  Oprp => "Oprp",              // + or -
		  OprlOr => "OprlOr",            // or
		  OprlAnd => "OprlAnd",           // and
		  OprlNot => "OprlNot",           // not
		  OprPP => "OpPP",             // ++
		  OprlrEq => "OprlrEq",           // operator equals or not equals
		  OprlrLgt => "OprlrLgt",          // operator larger greater or less than
		  OprlrLgtEq => "OprlrLgtEq",        // operator larger greater or less than
		  NumericConst => "NumericConst",      // 1.29
		  AttrTo => "AttrTo",            // =
		  CharConst => "CharConst",         // 'a'
		  StringConst => "StringConst",       // "lol"
		  OpBrackets => "OpBrackets",        // [
		  ClBrackets => "ClBrackets",        // ]
		  OpParenthesys => "OpBrackets",     // (
		  ClParenthesys => "ClParenthesys",     // )
		  OpCurlyBrackets => "OpCurlyBrackets",   // {
		  ClCurlyBrackets => "ClCurlyBrackets",   // }
		  Comma => "Comma",             //  => "",
		  Semicolon => "Semicolon"         // ;*/
		  _ => write!(f, "lol"),
		}
    }
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Token
{
  pub line: usize,
  pub col: usize,
  pub kind: TokenTypes,
  pub inverse: bool,
  pub str_tk: String,
}
