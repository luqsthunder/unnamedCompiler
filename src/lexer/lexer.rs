use token::tokens::Token;
use code_file::source_code::SourceCode;

pub struct Lexer
{
  tokens_list: Vec<Token>,
  current_pos: (u64, u64),
  src: SourceCode,
}

impl Lexer
{
  pub fn new(src_code: SourceCode) -> Lexer
  {
    let ret = Lexer{tokens_list = Vec!<Token>, current_pos = (0, 0), 
                    src: src_code};

    return ret;
  }

  pub fn run(self: Lexer)
  {
    
  }

  fn next_token(self: Lexer) -> Token
  {
    
  }

  fn create_line_list(self: Lexer) -> Vec<String>
  {
    
  }

}
