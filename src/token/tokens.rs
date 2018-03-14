//! keywords: 
//!    types: int, float, char, vec types-> int::, string(char::), float::
//!    statement keywords: if, else
//!

pub enum TokenTypes
{
  ID{id: u8, name: str},              // ID of a var, const value
  TypeInt{id: u8, name: str},         // type integer
  TypeFloat{id: u8, name: str},       // type float
  TypeChar{id: u8, name: str},        // type character
  TypeVec{id: u8, name: str},         // type vector
  Expr{id: u8, name: str},            // expression with a boolean value
  Statement{id: u8, name: str},       // the body of things like function { here is a statement }
  Fndef{id: u8, name: str},           // function token
  FnCall{id: u8, name: str},          // function call token
  FnParam{id: u8, name: str},         // function param token
  RetKey{id: u8, name: str},          // return keyword
  IfKey{id: u8, name: str},           // if keyword
  ElseKey{id: u8, name: str},         // keyword else
  EqOpr{id: u8, name: str},           // operator equals or not equals
  LgtOpr{id: u8, name: str},          // operator larger greater or less than
  OpBrackets{id: u8, name: str},      // [
  ClBrackets{id: u8, name: str},      // ]
  OpParenthesys{id: u8, name: str},   // (
  ClParenthesys{id: u8, name: str},   // )
  OpCurlyBrackets{id: u8, name: str}, // {
  ClCurlyBrackets{id: u8, name: str}, // }
}

pub fn print_token(tk_type: TokenTypes)
{
  match tk_type
  {
    ID => println(""),
    TypeInt => println(""),
    TypeFloat => println(""),
    TypeChar => println(""),
    TypeVec => println(""),
    Statement => println(""),
    RetKey => println(""),
    IfKey => println(""),
    ElseKey => println(""),
    EqOpr => println(""),
    LgtOpr => println(""),
    OpBrackets => println(""),
    ClBrackets => println(""),
    OpParenthesys => println(""),
    ClParenthesys => println(""),
    OpCurlyBrackets => println(""),
    ClCurlyBrackets => println(""),
  }
}

pub struct Token
{
  line: u64,
  col: u64,
  kind: TokenTypes,
}