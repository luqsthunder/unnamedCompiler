pub enum TokenTypes
{
  type_int,
  type_float,
  type_char,
  type_vec,
  expr,
  ret_fun,
  ret_key,
  if_key,
  else_key,
  eq_opr,
  lgt_opr,
}

#[derive(Debug)]
struct SourceCode
{
  sourceContent: String,
}

impl SourceCode
{

fn new(file_path: String) -> Result<SourceCode>
{
  let mut src = SourceCode{ sourceContent: String::new()};

  let mut err = Err();

  let mut file = match File::open(file_path)
  {
    Ok(t) => t,
    Err(_) => ,
  };

  match file.read_to_string(&mut src.sourceContent)
  {
    Ok(t) => t,
    Err(_) => Err("Can't read file" + file_path),
  };

  return src;
}

fn removeNewLines(self:SourceCode, fileContent: &mut str)
{
  for c in self.sourceContent
  {
  }
}

fn tokenStractor(fileContent: &str)
{

}

}
