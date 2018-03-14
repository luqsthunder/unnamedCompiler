extern crate regex;

use std::env;
mod code_file;
use code_file::source_code::SourceCode;
fn main()
{
  let file_name = match env::args().nth(1)
  {
    Some(t) => t,
    _ => panic!("no file passed as arg"),
  };

  let sourceCode = match SourceCode::new(file_name)                            
  {
    Ok(t) => t,
    Err(e) => panic!(e.message),
  };

  
}
