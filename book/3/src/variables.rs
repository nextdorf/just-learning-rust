pub fn main_vars(){
  let mut x = 6;
  println!("x: {x}");
  x = 5;
  println!("x: {x}");

  let mut y = String::new();
  y += "ABC";
  println!("y: {y}");
  y = String::new();
  y += "XYZ";
  println!("y: {y}");

  const THREE_HOURS_IN_SECONDS: u32 = 60*60*3;
  println!("3h in secs: {}", THREE_HOURS_IN_SECONDS);

  let z = 5;
  let z = z+1;
  {
    let z = z+1;
    println!("z (inner): {z}");
  }
  println!("z (outer): {z}");

  let spaces = "      ";
  print!("The string \"{spaces}\" contains ");
  let spaces = spaces.len();
  println!("{spaces} characters");

}

