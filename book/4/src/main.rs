use std::io::stdin;

fn main() {
  let mut s = String::from("Hello");
  s += ", world";
  println!("{s}");
  let s2 = s.as_str();
  
  //Both paths can either be true or false, but both can't be true
  let (path1, _path2) = (true, false);
  if path1
    {drop(s);}
  else
  //if _path2
    {println!("{s2}");}


  let s = String::from("Ownership moved for String");
  println!("{s}");
  let s2 = s;
  //println!("{s}"); //Error
  println!("{s2}"); //Fine


  let s = "No ownership problems for str";
  println!("{s}");
  let s2 = s;
  println!("{s}"); //Fine
  println!("{s2}"); //Also Fine


  let s1 = String::from("Ownership moved for Tuples and Strings");
  let s2 = "Ownership not moved for Tuples and str";
  let t = (s1, s2);
  //println!("{}", s1); //Error
  println!("{}", t.1); //Fine    
  println!("{}", s2); //Fine    


  let s = String::from("Deep copy with clone");
  let _t = (1, s.clone());
  println!("{}", s); //Fine

  let arr = (String::from("skjadhk"), [1,2,3], ["kashd", "kdsh"]);
  let _brr = arr.clone();
  println!("{arr:?}");

  let arr = [1,2,3];
  let _brr = arr;

  let s = String::from("sadj");
  otherfn(s.as_str());
  otherfn(s.as_str());
  let s = otherfn2(s);
  let s = &&&&&&&&&&&&s;
  println!("{}", s.as_str());

  otherfn3(s);

  let s = String::from("sadj");
  let t = (1, &s);
  let t = (t.0, "No ownership");
  otherfn2(s);
  println!("{t:?}");


  let mut s = String::from("saoda");
  otherfn3(&&s);
  otherfn4(&mut s);
  let s_ref = &mut s;
  //otherfn4(&mut s); //Error
  println!("{}", s_ref);

  println!("\nGet first word of:");
  let mut input = String::new();
  stdin().read_line(&mut input).expect("Invalid input");
  let word1 = first_word(&input);
  println!("\nFirst word is: \"{}\"", word1);
  //assert_eq!(word1, "hello");

  let a = [1, 2, 3, 4, 5];
  let slice = &a[1..3];
  assert_eq!(slice, &[2, 3]);
  println!("Array equality: {}", slice == &[2, 3]);

}


fn otherfn(_x: &str){}

fn otherfn2(x: String) -> String { x }

fn otherfn3(x: &&String){
// fn otherfn3(x: &&str){
  let _y = x;
  let _y = *x;
}

fn otherfn4(x: &mut String) {
  *x += "sadasd";
}

// fn dangle() -> &String {
//   let s = String::from("asdhajksd");
//   &s
// }

// fn first_word(s: &String) -> &str {
fn first_word(s: &str) -> &str {
  for (i, &c) in s.trim().as_bytes().iter().enumerate(){
    if c == b' '
      {return &s[..i];} 
  }
  &s[..]
}
