use std::io;

pub fn main_datatypes(){
  let c = 'z';
  let z: char = 'ℤ'; // with explicit type annotation
  let heart_eyed_cat = '😻';

  println!("Legal chars: {c}, {z}, {heart_eyed_cat}");

  let tuple0: (i32, bool, char) = (1, true, '👍');
  let (_i0, _b0, c0) = tuple0;
  let i0 = tuple0.0;
  println!("Emoji: {c0}, number: {i0}");

  let arr = [42, 1337, 420];
  println!("Array: [{}, {}, {}]", arr[0], arr[1], arr[2]);

  let arr: [i32; 3];
  arr = [1,2,3];    //fine
  //arr = [3,4,5];  //error
  println!("Array: [{}, {}, {}]", arr[0], arr[1], arr[2]);

  let q: i32;
  q = 7;
  println!("q: {q}, value assigned after declaration");

  let arr = [1,2,3,4,5];
  println!("> Enter index for array:");
  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("Error during stdin");
  let idx: usize = input.trim().parse().expect("Could not parse input");
  println!("arr[input] = {}", arr[idx]);
}

