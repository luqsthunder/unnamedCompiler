use std::fs::File;
use std::io::Read;
use std::string::String;

#[derive(Debug)]
struct SourceCode
{
  sourceContent: String,
}

impl SourceCode
{

fn new(file_path: String) -> Result<SourceCode, std::error::Err> 
{
  let mut src = SourceCode{ sourceContent: String::new()};

  let mut file = match File::open(file_path)
  {
    Ok(t) => t,
    Err(_) => 
    {
      let str_error = String::from("cant open file: ");
      str_error.push(file_path)
      return Err(str_error);
    },
  };

  match file.read_to_string(&mut src.sourceContent)
  {
    Ok(t) => t,
    Err(_) => 
    {
      let str_error = String::from("cant read file: ");
      str_error.push(file_path)
      return Err(str_error);
    },
  };

  return  src;
}

fn removeNewLines(self:SourceCode, fileContent: &mut str)
{
  self.sourceContent.as_mut_vec().retain(|c| c != ('\n' as u8));
  println!("{}", self.sourceContent);
}

fn tokenStractor(fileContent: &str)
{

}

}