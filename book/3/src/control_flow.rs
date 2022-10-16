use std::io;

pub fn main_control_flow(){
  let b = true;
  let x = {
    if b {1} else {2}
  };

  println!("Check if divisible by 4");
  let input:i32 = read_and_parse();
  println!("{input} is divisible by 4: {}", input % 4 == 0);
  
  let mut i=0;
  let x = loop{
    if i>= 10{
      break i*2
    }
    i+=1;
  };
  println!("increase i up to 10 and double: {x}");

  nested_loops();

  for x in [1,2,3]{
    print!("{x} ")
  }
  let mut arr = [1,2,3];
  for x in arr.iter_mut() { *x *= *x; };
  let arr = arr;
  println!("\n{arr:?}");

  let brr = [(1,'a'), (2,'b')].map(|(x, _y)| x*10);
  println!("{brr:?}");

  println!("Countdown from:");
  for number in (1..=read_and_parse()).rev() {
    print!("{number} ");
  }
  println!("... LIFTOFF!!!");

}

fn nested_loops(){
  //
  let mut count = 0;
  'counting_up: loop { //labeled loop with name "'counting_up"
    println!("count = {count}");
    let mut remaining = 10;

    loop {
      println!("remaining = {remaining}");
      if remaining == 9 {
        break;
      }
      if count == 2 {
        break 'counting_up;
      }
      remaining -= 1;
    }

    count += 1;
  }
  println!("End count = {count}");

}

fn read_and_parse() -> i32 {
  let mut input = String::new();
  io::stdin()
    .read_line(&mut input)
    .expect("Error during stdin");
  //input.trim().parse().expect("Could not parse input")
  input.trim().parse().expect("Could not parse input")
}

