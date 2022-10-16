pub fn main_funcs(){
  print_sum(2,3);

  let y = {
    let x = 3;
    x + 1
  };
  println!("y = 3+1: {y}");

  println!("5: {}", five());
}

fn print_sum(a:i32, b:i32) -> i32{
  let ab = a+b;
  println!("{a}+{b}: {}", ab);
  ab
}

fn five() -> i32 {
  5
}

//comment1

/*
sadjsalkd
sdsakldjlk
asdalskd
sadjalksd
*/

