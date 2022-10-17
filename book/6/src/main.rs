fn main() {
  let msg = Message::Write(String::from("Hello, msg!"));
  msg.call();

  let some1 = Some(1);
  let some2 = opt_plus_one(some1);
  let some3 = if let Some(x) = some2 {x} else {0};
}

fn opt_plus_one(x: Option<i32>) -> Option<i32> {
  match x{
    Some(i) => Some(i+1),
    _ => None
  }
}

enum Message {
  Quit,
  Write(String),
  MoveTo {x: i32, y: i32},
  ChangeColor(i32, i32, i32),
}

impl Message {
  fn call(&self){
    match self{
      Self::Write(s) => println!("{}", s),
      Self::MoveTo { x: 0, y:_ } => println!("x=0"),
      _ => (),
    }
  }
}
