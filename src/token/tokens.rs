#![allow(dead_code)]
//! keywords:
//!    types: int, float, char, vec types-> int::, string(char::), float::
//!    statement keywords: if, else
//!

use std::fmt;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum TokenTypes {
    ID,
    // ID of a var, const value
    TypeInt,
    // type integer
    TypeFloat,
    // type float
    TypeChar,
    // type character
    VecDec,
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
    Oprm,
    // * or /
    Oprp,
    // + or -
    OprlLogic,
    // or and
    OprlNot,
    // not
    OpUnS,
    // ++ --
    OprlrRel,
    // operator equals or not equals == < > >= <= !=
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
    OpPar,
    // (
    ClPar,
    // )
    OpCBracks,
    // {
    ClCBracks,
    // }
    Comma,
    // vazio
    Epsilon,
    // ,
    Eof,
}


impl fmt::Display for TokenTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self
            {
                TokenTypes::ID => write!(f, "ID"),
                TokenTypes::TypeInt => write!(f, "TypeInt"),
                TokenTypes::TypeFloat => write!(f, "TypeFloat"),
                TokenTypes::TypeChar => write!(f, "TypeChar"),
                TokenTypes::VecDec => write!(f, "TypeVec"),
                TokenTypes::VoidKey => write!(f, "VoidKey"),
                TokenTypes::RetKey => write!(f, "RetKey"),
                TokenTypes::ForKey => write!(f, "ForKey"),
                TokenTypes::WhileKey => write!(f, "WhileKey"),
                TokenTypes::IfKey => write!(f, "IfKey"),
                TokenTypes::ElseKey => write!(f, "ElseKey"),
                TokenTypes::Oprm => write!(f, "Oprm"),
                TokenTypes::Oprp => write!(f, "Oprp"),
                TokenTypes::OprlLogic => write!(f, "OprlrLogic"),
                TokenTypes::OprlNot => write!(f, "OprlNot"),
                TokenTypes::OpUnS => write!(f, "OprUnSM"),
                TokenTypes::OprlrRel => write!(f, "OprlrRel"),
                TokenTypes::FloatConst => write!(f, "FloatConst"),
                TokenTypes::AttrTo => write!(f, "AttrTo"),
                TokenTypes::CharConst => write!(f, "CharConst"),
                TokenTypes::StrConst => write!(f, "StrConst"),
                TokenTypes::OpPar => write!(f, "OpBracks"),
                TokenTypes::ClPar => write!(f, "ClPar"),
                TokenTypes::OpCBracks => write!(f, "OpCBracks"),
                TokenTypes::ClCBracks => write!(f, "ClCBracks"),
                TokenTypes::Comma => write!(f, "Comma"),
                TokenTypes::Epsilon => write!(f, "Epsilon"),
                TokenTypes::Eof => write!(f, "EOF"),
                TokenTypes::TypeStr => write!(f, "TypeStr"),
                TokenTypes::IntConst => write!(f, "IntConst"),
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
    pub lex: String,
}
