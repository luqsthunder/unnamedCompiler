//! keywords:
//!    types: int, float, char, vec types-> int::, string(char::), float::
//!    statement keywords: if, else
//!

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
  Err,               // only for pattern matching
  RetKey,            // return keyword
  ForKey,            // for keyword
  WhileKey,          // while keyword
  IfKey,             // if keyword
  ElseKey,           // keyword else
  Dread,             // # comment
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

#[derive(Debug)]
pub struct Token
{
  pub line: usize,
  pub col: usize,
  pub kind: TokenTypes,
  pub neg: bool,
  pub str_tk: String,
}
