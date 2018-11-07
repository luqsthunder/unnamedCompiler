extern crate regex;

use token::tokens::Token;
use lexer::lexemes;
use token::tokens::TokenTypes;

use std::vec;
use std::fs::File;
use std::io::{Seek, SeekFrom, Read, ErrorKind};
use std::string::String;
use std::collections::HashMap;

#[derive(Debug)]
pub enum LexerErrorType {
    OpenFile,
    ReadFile,
    UnknowCharSeq,
}

#[derive(Debug)]
pub struct LexerError {
    pub cause: LexerErrorType,
    pub message: String,
}

#[derive(PartialEq)]
enum LexGroup {
    Empty,
    AlphaNumeric,
    Numeric,
    Symbol,
}

pub struct Lexer {
    src: File,
    line_pos: usize,
    col_pos: usize,
}

fn tk_type_from_str(s: &String) -> Result<TokenTypes, LexerError> {
    let m = s.clone();
    println!("str come: {}", s.clone());
    match m.as_ref() {
        "for" => Ok(TokenTypes::ForKey),
        "while" => Ok(TokenTypes::WhileKey),
        "if" => Ok(TokenTypes::IfKey),
        "else" => Ok(TokenTypes::ElseKey),
        "return" => Ok(TokenTypes::RetKey),

        "::" => Ok(TokenTypes::VecDec),

        "or" | "and" => Ok(TokenTypes::OprlLogic),
        "=>" | "=="| "<=" |"!=" | ">" | "<" => Ok(TokenTypes::OprlrRel),

        "not" => Ok(TokenTypes::OprlNot),

        "--" | "++" => Ok(TokenTypes::OpUnS),

        "void" => Ok(TokenTypes::VoidKey),
        "int" => Ok(TokenTypes::TypeInt),
        "char" => Ok(TokenTypes::TypeChar),
        "float" => Ok(TokenTypes::TypeFloat),
        "string" => Ok(TokenTypes::TypeStr),
        "::" => Ok(TokenTypes::VecDec),

        "{" => Ok(TokenTypes::OpCBracks),
        "}" => Ok(TokenTypes::ClCBracks),
        "(" => Ok(TokenTypes::OpPar),
        ")" => Ok(TokenTypes::ClPar),
        "," => Ok(TokenTypes::Comma),

        "=" => Ok(TokenTypes::AttrTo),

        "+" | "-" => Ok(TokenTypes::Oprp),
        "*" | "/" => Ok(TokenTypes::Oprm),

        _ => {
            Ok(TokenTypes::Epsilon)
        },
    }
}

impl Lexer {
    pub fn new(file_path: String) -> Result<Lexer, LexerError> {
        let mut file = match File::open(&file_path) {
            Ok(t) => t,
            Err(err) => {
                println!("{}", err);
                let error = LexerError {
                    message: String::from("cant open file: "),// + file_path.as_str(),
                    cause: LexerErrorType::OpenFile,
                };
                return Err(error);
            },
        };
        let mut ret = Lexer { src: file, col_pos: 1, line_pos: 1 };

        return Ok(ret);
    }

    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        let mut token_string: String = String::new();
        token_string.reserve(10);

        let mut s_char: Vec<u8> = Vec::with_capacity(1);
        s_char.push(0);

        let mut block_comment: u32 = 0;
        let mut string_block: bool = false;
        let mut lex_group = LexGroup::Empty;
        let mut tk_type = TokenTypes::Epsilon;

        loop {
            let read_size = match self.src.read_exact(s_char.as_mut()) {
                Ok(t) => t,
                Err(e) => {
                    if e.kind() == ErrorKind::UnexpectedEof {
                        if token_string.is_empty() {
                            return Ok(Token {
                                line: self.line_pos,
                                col: self.col_pos,
                                kind: TokenTypes::Eof,
                                lex: String::from("")
                            });
                        } else {
                            break;
                        }
                    }
                    panic!(e);
                },
            };

            self.line_pos += 1;
            if s_char[0] == ('\n' as u8) {
                if token_string.is_empty() {
                    continue;
                }
                break;
            }
            if string_block {
                if s_char[0] == ('"' as u8) {
                    break;
                }
                token_string.push(s_char[0] as char);
            }
            // alpha-numeric tokens e.g id, int, numbers
            if (s_char[0] >= ('0' as u8) && s_char[0] <= ('9' as u8)) ||
               (s_char[0] >= ('a' as u8) && s_char[0] <= ('z' as u8)) ||
               (s_char[0] >= ('A' as u8) && s_char[0] <= ('Z' as u8)) ||
               (s_char[0] == ('_' as u8)) {
                if lex_group != LexGroup::AlphaNumeric && lex_group != LexGroup::Empty {
                    self.src.seek(SeekFrom::Current(-1));
                    break;
                }

                token_string.push(s_char[0] as char);
                lex_group = LexGroup::AlphaNumeric;
                continue;
            }
            // lê tudo que for simbolo para quando achar um espaço ou algo
            // que não seja simbolo, quando achar algo que não seja simbolo
            // ele volta 1 caracter na leitura
            // e.g:  ==x isso para quando achar x,
            else if s_char[0] != (' ' as u8) {
                if s_char[0] == ('"' as u8) {
                    string_block = true;
                }
                if lex_group != LexGroup::Symbol && lex_group != LexGroup::Empty {
                    self.src.seek(SeekFrom::Current(-1));
                    break;
                }


                if !token_string.is_empty() && (s_char[0] as char) == '(' {
                    self.src.seek(SeekFrom::Current(-1));
                    break;
                }

                token_string.push(s_char[0] as char);
                lex_group = LexGroup::Symbol;
                continue
            }

            if !token_string.is_empty() {
                break;
            }
        }

        tk_type = match tk_type_from_str(&token_string) {
            Ok(t) => t,
            Err(e) => TokenTypes::Epsilon,
        };

        let tk_ok = Ok(Token {
            line: self.line_pos,
            col: self.col_pos,
            kind: tk_type.clone(),
            lex: token_string
        });
        if s_char[0] == '\n' as u8 {
            self.line_pos = 1;
            self.col_pos += 1;
        }

        tk_ok
    }

    pub fn print_token_as_alcino_likes(tk: &Token)
    {
        let kt = tk.clone();
        let kind_value = kt.kind.clone() as u8;
        let kind_str = kt.kind.to_string();
        println!("[{:#03}, {:#03}] ({:#04}, {:10}) {{{}}}", kt.line, kt.col,
                 kind_value, kind_str, kt.lex);
    }
}
