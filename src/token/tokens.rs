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
  ID,
  // ID of a var, const value
  TypeInt,
  // type integer
  TypeFloat,
  // type float
  TypeChar,
  // type character
  TypeVec,
  // type vector
  TypeStr,
  // type string
  VoidKey,
  // void keyword
  RetKey,
  // return keyword
  ForKey,
  // for keyword
  WhileKey,
  // while keyword
  IfKey,
  // if keyword
  ElseKey,
  // keyword else
  PrintFun,
  // print func
  Comment,
  // # comment
  Oprm,
  // * or /
  Oprp,
  // + or -
  OprlOr,
  // or
  OprlAnd,
  // and
  OprlNot,
  // not
  OprPP,
  // ++
  OprlrEq,
  // operator equals or not equals
  OprlrLgt,
  // operator larger greater or less than
  OprlrLgtEq,
  // operator larger greater or less than
  FloatConst,
  // 1.29
  IntConst,
  // 123123
  AttrTo,
  // =
  CharConst,
  // 'a'
  StrConst,
  // "lol"
  OpBracks,
  // [
  ClBracks,
  // ]
  OpPar,
  // (
  ClPar,
  // )
  OpCBracks,
  // {
  ClCBracks,
  // }
  Comma,
  // ,
  Semicolon,
  // ;
  Eof,
}


impl fmt::Display for TokenTypes {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self
    {
      TokenTypes::ID         => write!(f, "ID"),
      TokenTypes::TypeInt    => write!(f, "TypeInt"),
      TokenTypes::TypeFloat  => write!(f, "TypeFloat"),
      TokenTypes::TypeChar   => write!(f, "TypeChar"),
      TokenTypes::TypeVec    => write!(f, "TypeVec"),
      TokenTypes::VoidKey    => write!(f, "VoidKey"),
      TokenTypes::RetKey     => write!(f, "RetKey"),
      TokenTypes::ForKey     => write!(f, "ForKey"),
      TokenTypes::WhileKey   => write!(f, "WhileKey"),
      TokenTypes::IfKey      => write!(f, "IfKey"),
      TokenTypes::ElseKey    => write!(f, "ElseKey"),
      TokenTypes::PrintFun   => write!(f, "PrintFun"),
      TokenTypes::Comment    => write!(f, "Comment"),
      TokenTypes::Oprm       => write!(f, "Oprm"),
      TokenTypes::Oprp       => write!(f, "Oprp"),
      TokenTypes::OprlOr     => write!(f, "OprlOr"),
      TokenTypes::OprlAnd    => write!(f, "OprlAnd"),
      TokenTypes::OprlNot    => write!(f, "OprlNot"),
      TokenTypes::OprPP      => write!(f, "OpPP"),
      TokenTypes::OprlrEq    => write!(f, "OprlrEq"),
      TokenTypes::OprlrLgt   => write!(f, "OprlrLgt"),
      TokenTypes::OprlrLgtEq => write!(f, "OprlrLgtEq"),
      TokenTypes::FloatConst => write!(f, "FloatConst"),
      TokenTypes::AttrTo     => write!(f, "AttrTo"),
      TokenTypes::CharConst  => write!(f, "CharConst"),
      TokenTypes::StrConst   => write!(f, "StrConst"),
      TokenTypes::OpBracks   => write!(f, "OpBrackets"),
      TokenTypes::ClBracks   => write!(f, "ClBracks"),
      TokenTypes::OpPar      => write!(f, "OpBracks"),
      TokenTypes::ClPar      => write!(f, "ClPar"),
      TokenTypes::OpCBracks  => write!(f, "OpCBracks"),
      TokenTypes::ClCBracks  => write!(f, "ClCBracks"),
      TokenTypes::Comma      => write!(f, "Comma"),
      TokenTypes::Semicolon  => write!(f, "Semicolon"),
      TokenTypes::Eof        => write!(f, "EOF"),
      TokenTypes::TypeStr    => write!(f, "TypeStr"),
      TokenTypes::IntConst   => write!(f, "IntConst"),
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
