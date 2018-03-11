use std::fs::File;
use std::env;
use std::io::Read;

fn main()
{
  let file_name = match env::args().nth(1)
  {
    Some(t) => t,
    _ => panic!("no file passed as arg"),
  };

  
}
