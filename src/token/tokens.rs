//! keywords: 
//!    types: int, float, char, vec types-> int::, string(char::), float::
//!    statement keywords: if, else
//!

pub enum TokenTypes
{
  ID,              // ID of a var, const value
  TypeInt,         // type integer
  TypeFloat,       // type float
  TypeChar,        // type character
  TypeVec,         // type vector
  Expr,            // expression with a boolean value
  Statement,       // the body of things like function { here is a statement }
  Fndef,           // function token
  FnCall,          // function call token
  FnParam,         // function param token
  RetKey,          // return keyword
  IfKey,           // if keyword
  ElseKey,         // keyword else
  EqOpr,           // operator equals or not equals
  LgtOpr,          // operator larger greater or less than
  OpBrackets,      // [
  ClBrackets,      // ]
  OpParenthesys,   // (
  ClParenthesys,   // )
  OpCurlyBrackets, // {
  ClCurlyBrackets, // }
}
