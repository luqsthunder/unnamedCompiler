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
  Err,               // only for pattern matching errors
  RetKey,            // return keyword
  ForKey,            // for keyword
  WhileKey,          // while keyword
  IfKey,             // if keyword
  ElseKey,           // keyword else
  Dread,             // # comment
  Oprm,              // * or /
  Oprp,              // + or -
  OprlrEq,           // operator equals or not equals
  OprlrLgt,          // operator larger greater or less than
  OprlrLgtEq,        // operator larger greater or less than
  FloatConst,        // 1.29
  CharConst,         // 'a'
  IntConst,          // 123
  StringConst,       // "lol"
  OpBrackets,        // [
  ClBrackets,        // ]
  OpParenthesys,     // (
  ClParenthesys,     // )
  OpCurlyBrackets,   // {
  ClCurlyBrackets,   // }
  Comma,             // ,
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
