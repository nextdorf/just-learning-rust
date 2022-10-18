use std::{io::{stdin, ErrorKind}, fs::File};
use std::error::Error;

//you can read Box<dyn Error> to mean “any kind of error.”
fn main() -> Result<(), Box<dyn Error>> {
  println!("Path to open");
  let mut buf = String::new();
  stdin().read_line(&mut buf).expect("");

  let f_res = File::open(buf.trim());
  let f = f_res
    .unwrap_or_else(|err| {
      if err.kind() == ErrorKind::NotFound {
        File::create(buf.trim())
          .unwrap_or_else(|e| panic!("FILE-ERROR: {}", e))
      }
      else { panic!("FILE-ERROR: {}", err) }
    });

  let text = "";//"abc\ndef\n123";
  println!("{}\n -> {}", text, last_char_of_first_line(text).unwrap_or('𓀑'));

  if let Ok(f) = File::open("pfad.txt") {
    println!("{:?}", f)
  }
  if let Some(l1) = "zeile1\nzeile2".lines().next() {
    println!("{}", l1)
  }
  Ok(())
}

fn last_char_of_first_line(text: &str) -> Option<char> {
  text.lines().next()?.chars().last()
}
