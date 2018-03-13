use std::fs::File;
use std::io::Read;
use std::string::String;

#[derive(Debug)]
pub struct SourceCode
{
  src_content: String,
  /*src: String,
  srcLines: Vec<String>,*/
}

 #[derive(Debug)]
pub enum SrcCodeCause
{
  OPEN,
  READ,
}

pub struct SourceCodeErr
{
  pub message: String,
  pub cause: SrcCodeCause,
}

impl SourceCode
{

pub fn new(file_path: String) -> Result<SourceCode, SourceCodeErr>
{
  let mut src = SourceCode{ src_content: String::new()};

  let mut file = match File::open(file_path)
  {
    Ok(t) => t,
    Err(_) => 
    {
      let error = SourceCodeErr { 
        message: String::from("cant open file: "),
        cause: SrcCodeCause::OPEN,
      };
      return Err(error);
    },
  };

  match file.read_to_string(&mut src.src_content)
  {
    Ok(t) => t,
    Err(_) => 
    {
      let error = SourceCodeErr { 
        message: String::from("cant read file: "),
        cause: SrcCodeCause::READ,
      };
      return Err(error);
    },
  };

  return  Ok(src);
}

//! this function removes new lines and spaces between terminals and initials 
fn remove_new_lines(self:SourceCode)
{
  
}

fn token_stractor(file_content: &str)
{

}

}