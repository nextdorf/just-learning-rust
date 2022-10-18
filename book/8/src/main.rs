use std::collections::HashMap;

fn main() {
  // let v = vec![1,2,3];
  // let v0 = v[0];
  // let v0_ = v.get(0);
  // println!("{v:?}");
  let mut v = Vec::new();
  for i in 1..=10{
    v.push(i);
  }
  let v0 = &v[0];
  println!("{v:?}, {v0}");

  let mut v = vec![1,2,3,4];
  for vi in &mut v{
    *vi *= *vi;
  }
  println!("{v:?}");
  println!();

  for s in ["السلام عليكم", "Dobrý den", "Hello", "שָׁלוֹם", 
    "नमस्ते", "こんにちは", "안녕하세요", "你好", "Olá", "Здравствуйте", 
    "Hola"]{
      println!("{s}");
  }
  println!();

  let mut s = "hsfjkfhkjsd".to_string();
  s.push_str(" sadklja");
  s += " zzz";
  s.push('Q');

  let (s1, s2) = ("str1".to_string(), "str2".to_string());
  let _s3 = s1 + &s2;

  let mut hello_cyrillic = "Здравствуйте".to_string();
  let hello_cyrillic_sliced = hello_cyrillic
    .chars().take(2).collect::<String>();
  hello_cyrillic += " ...";
  println!("\"{}\"[:2] = \"{}\"", hello_cyrillic, hello_cyrillic_sliced);
  println!();

  let mut scores = HashMap::new();
  scores.insert("team1".to_string(), 10);
  scores.insert("team2".to_string(), 50);
  let team2_score = scores.get("team2").copied().unwrap_or(0);
  println!("team2[score] = {team2_score}");
  println!("All scores:");
  for (t, s) in &scores{
    println!("  {}: {}", t, s);
  }
  scores.entry("team1".to_string())
    .and_modify(|e| {*e *=2})
    .or_insert(0);
  println!("team1[score] = {}", scores.get("team1").unwrap());
  println!();

  let text = "hello world wonderful world";
  let mut map = HashMap::new();
  for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
  }
  println!("Count word in \"{}\":\n {:?}", text, map);

}
